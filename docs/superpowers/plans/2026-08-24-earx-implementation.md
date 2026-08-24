# earX Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a cross-platform desktop application (Tauri v2 + Rust + React/TypeScript) to monitor and control Nothing and CMF earbuds (specifically CMF Buds Pro 2 / CMF Buds 2 Plus, Nothing Ear, Nothing Ear (2), CMF Buds, etc.) over Bluetooth SPP/RFCOMM on Windows and macOS.

**Architecture:** A lightweight Rust backend handles raw binary packet framing, CRC16 Modbus calculations, platform-specific Bluetooth RFCOMM connections, and state management actor loops. Tauri v2 IPC exposes commands and streams state updates to a React 18 frontend with an authentic Nothing OS industrial design system.

**Tech Stack:** Tauri v2, Rust 1.78+ (`tokio`, `serialport`, `windows`, `thiserror`, `tracing`, `serde`), React 18, Vite, TypeScript, Tailwind CSS, Lucide icons, Zustand.
**Goal:** Build a cross-platform desktop application (Tauri v2 + Rust + React/TypeScript) matching the authentic Nothing OS / Nothing X mobile companion app design for controlling Nothing and CMF earbuds (specifically CMF Buds Pro 2 / CMF Buds 2 Plus, Nothing Ear, Nothing Ear (2), CMF Buds, etc.) over Bluetooth SPP/RFCOMM on Windows and macOS. Always resident in the system tray / menu bar with an active earbud icon and launch-at-startup support.

**Architecture:** A lightweight Rust backend handles raw binary packet framing, CRC16 Modbus calculations, platform-specific Bluetooth RFCOMM connections, system tray lifecycle, and state management actor loops. Tauri v2 IPC exposes commands and streams state updates to a React 18 frontend featuring the exact Nothing OS 2-column bento card grid, serif typography, circular noise control dials, 4-step ANC level slider, Ultra Bass red level meter, and battery telemetry bars.

**Tech Stack:** Tauri v2, `tauri-plugin-autostart`, Rust 1.78+ (`tokio`, `serialport`, `windows`, `thiserror`, `tracing`, `serde`), React 18, Vite, TypeScript, Tailwind CSS, Lucide icons, Zustand.
---

### Task 1: Project Scaffolding (Tauri v2 + React + Vite + TypeScript + Tailwind CSS)

**Files:**
- Create: `package.json`
- Create: `tsconfig.json`
- Create: `vite.config.ts`
- Create: `tailwind.config.js`
- Create: `postcss.config.js`
- Create: `index.html`
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/tauri.conf.json`
- Create: `src-tauri/src/main.rs`
- Create: `src-tauri/src/lib.rs`

**Interfaces:**
- Produces: Initialized Tauri v2 workspace that builds cleanly on Windows via `cargo check` and `pnpm build`.

- [ ] **Step 1: Create frontend package configuration**
Define `package.json` with dependencies: `@tauri-apps/api`, `@tauri-apps/plugin-shell`, `react`, `react-dom`, `zustand`, `lucide-react`, `clsx`, `tailwind-merge`, and devDependencies: `typescript`, `vite`, `@vitejs/plugin-react`, `tailwindcss`, `postcss`, `autoprefixer`.

- [ ] **Step 2: Create Vite and Tailwind configurations**
Configure `vite.config.ts`, `tailwind.config.js` with Nothing OS theme colors (`#121212`, `#21201f`, `#d71920`), and `index.html`.

- [ ] **Step 3: Scaffold Rust Tauri backend (`src-tauri/`)**
Configure `src-tauri/Cargo.toml` with dependencies: `tauri = { version = "2", features = ["tray-icon"] }`, `tokio = { version = "1", features = ["full"] }`, `serde = { version = "1", features = ["derive"] }`, `serde_json = "1"`, `thiserror = "1"`, `tracing = "0.1"`, `tracing-subscriber = "0.3"`, `async-trait = "0.1"`.

- [ ] **Step 4: Verify build scaffolding**
Run: `pnpm install` and `cd src-tauri && cargo check`
Expected: PASS with no compilation errors.

---

### Task 2: Rust Protocol Engine — Packet Framing, CRC16, and Float32 Custom EQ

**Files:**
- Create: `src-tauri/src/protocol/mod.rs`
- Create: `src-tauri/src/protocol/crc.rs`
- Create: `src-tauri/src/protocol/packet.rs`
- Create: `src-tauri/src/protocol/eq_float.rs`
- Create: `src-tauri/tests/protocol_tests.rs`

**Interfaces:**
- Produces:
  - `crc::compute_crc16(data: &[u8]) -> u16`
  - `packet::encode_frame(command: u16, seq: u8, payload: &[u8]) -> Vec<u8>`
  - `packet::decode_frame(raw: &[u8]) -> Result<ParsedFrame, ProtocolError>`
  - `eq_float::format_float_for_eq(val: f32, is_preamp: bool) -> [u8; 4]`
  - `eq_float::from_format_float_for_eq(bytes: [u8; 4]) -> f32`

- [ ] **Step 1: Write failing unit tests for CRC, framing, and float conversions**
Write test cases in `src-tauri/tests/protocol_tests.rs` matching known hex strings from `reference/res/js/bluetooth_socket.js`.

- [ ] **Step 2: Implement CRC16 Modbus algorithm**
Implement `compute_crc16` using polynomial `0xA001` with initial value `0xFFFF`.

- [ ] **Step 3: Implement Packet Encoder and Decoder**
Implement header generation (`[0x55, 0x60, 0x01, cmd_lsb, cmd_msb, len, 0x00, seq]`), payload appending, and CRC16 appending/verification.

- [ ] **Step 4: Implement Nothing EQ Float32 Byte Transformer**
Implement IEEE-754 reverse-endian float formatting and sign-bit inversion for Custom EQ gains.

- [ ] **Step 5: Run tests to verify they pass**
Run: `cargo test --test protocol_tests`
Expected: PASS.

---

### Task 3: Rust Device Database & SKU Model Identification

**Files:**
- Create: `src-tauri/src/models/mod.rs`
- Create: `src-tauri/src/models/device_info.rs`
- Create: `src-tauri/src/models/sku_map.rs`
- Create: `src-tauri/tests/sku_tests.rs`

**Interfaces:**
- Produces:
  - `sku_map::parse_serial_number(payload: &[u8]) -> Option<String>`
  - `sku_map::get_device_model_from_sku(sku: &str) -> Option<DeviceModelInfo>`
  - `sku_map::get_device_model_from_serial(serial: &str) -> Option<DeviceModelInfo>`

- [ ] **Step 1: Write unit tests for SKU mapping**
Test serial strings from CMF Buds Pro 2 (`SH...`), CMF Buds, Nothing Ear, and Ear (stick) (`MA...`).

- [ ] **Step 2: Implement Hardware and SKU lookup table**
Define `BaseId` (`B172`, `B168`, `B163`, `B171`, `B155`, `B162`, `B157`, `B174`, `B181`), product names, and feature capability flags.

- [ ] **Step 3: Implement Serial Number Decoder**
Decode serial number response payload (`0x4006`) containing comma-separated hardware identifiers.

- [ ] **Step 4: Run tests to verify**
Run: `cargo test --test sku_tests`
Expected: PASS.

---

### Task 4: Rust Bluetooth Transport Trait & Mock Simulator

**Files:**
- Create: `src-tauri/src/transport/mod.rs`
- Create: `src-tauri/src/transport/trait_def.rs`
- Create: `src-tauri/src/transport/mock.rs`
- Create: `src-tauri/tests/mock_transport_tests.rs`

**Interfaces:**
- Produces:
  - `BluetoothTransport` async trait (`scan_devices`, `connect`, `send`, `receive`, `disconnect`).
  - `MockBluetoothTransport` capable of simulating earbud responses (battery, ANC, EQ, SKU) for testing without hardware.

- [ ] **Step 1: Define `BluetoothTransport` trait**
Define the async transport trait using `async-trait`.

- [ ] **Step 2: Implement `MockBluetoothTransport`**
Implement stateful mock responding to `0xC006` (serial), `0xC007` (battery), `0xC01E` (ANC), etc.

- [ ] **Step 3: Write tests for Mock Transport communication loop**
Verify that sending commands to the mock yields the expected binary response streams.

---

### Task 5: Rust Platform Bluetooth Drivers (Windows & macOS)

**Files:**
- Create: `src-tauri/src/transport/windows_rfcomm.rs`
- Create: `src-tauri/src/transport/serial_port.rs`
- Create: `src-tauri/src/transport/macos_rfcomm.rs`

**Interfaces:**
- Produces:
  - `WindowsBluetoothTransport` (using WinRT RFCOMM socket and Serial COM fallback).
  - `MacBluetoothTransport` (using IOBluetooth / TTY fallback).
  - Factory function `create_platform_transport() -> Box<dyn BluetoothTransport>`.

- [ ] **Step 1: Implement Windows RFCOMM & COM port transport**
Implement device scanning via Windows Bluetooth APIs and connection via RFCOMM GUID `aeac4a03-dff5-498f-843a-34487cf133eb` / paired COM ports.

- [ ] **Step 2: Implement macOS transport abstraction**
Implement IOBluetooth / POSIX TTY stream abstraction with conditional compilation (`#[cfg(target_os = "macos")]`).

- [ ] **Step 3: Wire transport factory**
Return platform-appropriate transport or mock transport based on environment.

---

### Task 6: Rust State Machine Actor (`DeviceManager`) & Telemetry Engine

**Files:**
- Create: `src-tauri/src/state/mod.rs`
- Create: `src-tauri/src/state/device_state.rs`
- Create: `src-tauri/src/state/manager.rs`

**Interfaces:**
- Produces:
  - `DeviceState`: Struct holding live telemetry (left/right/case battery, charging states, ANC mode, EQ profile, in-ear state, bass level, firmware version).
  - `DeviceManager`: Actor managing connection lifecycle, initial sync sequence (`init_device`), periodic polling, and command dispatch.

- [ ] **Step 1: Define `DeviceState` struct with full serde serialization**
Model battery, ANC levels ($1 \dots 6$), EQ mode, enhanced bass ($0 \dots 5$), in-ear detection, low-latency state.

- [ ] **Step 2: Implement Actor loop in `DeviceManager`**
Implement command queue with $100\text{ms}$ inter-packet throttling and auto-reconnection logic on disconnect.

- [ ] **Step 3: Implement Telemetry Packet Parser**
Parse incoming raw packets into `DeviceState` updates.

---

### Task 7: Tauri IPC Commands & Real-Time Event Bridge

**Files:**
- Create: `src-tauri/src/commands.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/main.rs`

**Interfaces:**
- Produces:
  - Tauri commands: `scan_devices`, `connect_device`, `disconnect_device`, `set_anc_mode`, `set_eq_preset`, `set_custom_eq`, `set_bass_enhance`, `set_in_ear`, `set_latency_mode`, `ring_bud`, `get_state`.
  - Tauri event: `device-state-changed` emitted on every state mutation.

- [ ] **Step 1: Implement Tauri command handlers**
Write handlers forwarding requests to `DeviceManager`.

- [ ] **Step 2: Wire Tauri app setup with state and commands**
Register plugins, managed state, and commands in `src-tauri/src/lib.rs`.

- [ ] **Step 3: Verify backend compilation**
Run: `cd src-tauri && cargo check`
Expected: PASS.

---

### Task 8: Frontend Nothing OS Design System, NDot Fonts & Layout

**Files:**
- Create: `src/styles/globals.css`
- Create: `src/assets/fonts/` (copy verified open NDot / Space Grotesk font files from reference)
- Create: `src/components/layout/AppLayout.tsx`
- Create: `src/components/layout/Header.tsx`
- Create: `src/components/layout/Navigation.tsx`

**Interfaces:**
- Produces: Core dark/monochrome layout with dot-matrix headings, status bar, and page router.

- [ ] **Step 1: Set up fonts and typography in CSS**
Configure `@font-face` for NDot 55/57, Space Grotesk, and Roboto Mono.

- [ ] **Step 2: Build AppLayout and Header components**
Create glassmorphic header with device connection status badge, battery summary chip, and tab navigation.

---

### Task 9: Frontend Device Store (Zustand) & Tauri IPC Client

**Files:**
- Create: `src/types/device.ts`
- Create: `src/lib/tauri.ts`
- Create: `src/store/useDeviceStore.ts`

**Interfaces:**
- Produces:
  - Strongly typed frontend TypeScript definitions matching Rust `DeviceState`.
  - Zustand store listening to `device-state-changed` events and exposing action dispatchers.

- [ ] **Step 1: Define TypeScript interfaces**
Define `DeviceState`, `BatteryInfo`, `AncMode`, `EqPreset`, `DeviceModelInfo`.

- [ ] **Step 2: Implement Tauri IPC wrapper functions**
Implement strongly typed async wrappers calling `invoke("set_anc_mode", { mode })`, etc.

- [ ] **Step 3: Implement Zustand store**
Initialize store with listeners for real-time Tauri events.

---

### Task 10: Frontend Main Dashboard — Hero Visualizer, Battery Telemetry & Noise Control Dials

**Files:**
- Create: `src/components/dashboard/HeaderBar.tsx`
- Create: `src/components/dashboard/DualEarbudsVisualizer.tsx`
- Create: `src/components/dashboard/BatteryTelemetryBars.tsx`
- Create: `src/components/dashboard/ActionButtonsRow.tsx`
- Create: `src/components/dashboard/NoiseCancellationCard.tsx`
- Create: `src/components/dashboard/SpatialAudioCard.tsx`
- Create: `src/components/dashboard/BentoCardGrid.tsx`
- Create: `src/pages/DashboardPage.tsx`

**Interfaces:**
- Produces: Pixel-perfect replica of the Nothing X mobile companion app with the exact layout from the reference screenshots:
  - Header bar with back arrow, Serif "Device details", and pencil edit icon.
  - Dual photorealistic earbuds render matching device color.
  - Device title (e.g. "Ayush's CMF Buds 2 Plus") in Serif typography.
  - 3 battery progress bars (`— L 95%`, `⚡ — C 40%`, `— R 90%`).
  - Pill action buttons: `Forget` and `Disconnect`.
  - Noise cancellation card with 3 circular action dials (Noise cancellation, Transparency, Off) and 4-step segmented pills (`Low`, `Mid`, `High`, `Adaptive`).
  - Spatial audio card with `Fixed` and `Off` circular dials.
  - 2-Column Bento grid containing: Ultra Bass (toggle + 5-bar vertical red level meter), Personal Sound Profile, Equalizer, Controls, Low Lag Mode, Dual Connection, Device Settings, About, and System Settings.

- [ ] **Step 1: Build HeaderBar & DualEarbudsVisualizer**
Render top bar with back navigation and edit icon, plus high-res dual earbud graphic.

- [ ] **Step 2: Build BatteryTelemetryBars and ActionButtonsRow**
Render horizontal progress bars for Left, Case (with charging bolt), and Right buds, plus `Forget` and `Disconnect` buttons.

- [ ] **Step 3: Build NoiseCancellationCard with circular dials & 4-step segmented pills**
Circular mode buttons (`Noise cancellation`, `Transparency`, `Off`) with active solid white state, plus horizontal segmented pill selector (`Low`, `Mid`, `High`, `Adaptive`) when ANC is active.

- [ ] **Step 4: Build SpatialAudioCard and 2-Column BentoCardGrid**
Render Bento card grid featuring Ultra Bass (toggle + red meter bars), Equalizer, Controls, Low Lag Mode, Dual Connection, Device Settings, About, and System Settings.

- [ ] **Step 5: Assemble DashboardPage**
Wire all state and actions to `useDeviceStore`.

---

### Task 11: Frontend Subpages — Equalizer Hub & Wave Canvas

**Files:**
- Create: `src/components/equalizer/EqPresetPills.tsx`
- Create: `src/components/equalizer/WaveEqualizerCanvas.tsx`
- Create: `src/components/equalizer/AdvancedParametricEq.tsx`
- Create: `src/pages/EqualizerPage.tsx`

**Interfaces:**
- Produces: Subpage opened when clicking the Equalizer bento card, featuring preset pills (Balanced, More Bass, More Treble, Voice, Rock, Pop, Custom), interactive 3-band wave canvas, and 8-band parametric EQ for supported models.

- [ ] **Step 1: Build EqPresetPills**
Render preset selector pills with active white selection.

- [ ] **Step 2: Build WaveEqualizerCanvas**
Render interactive 3-band curve with draggable Bass, Mid, and Treble nodes.

- [ ] **Step 3: Build EqualizerPage with Back Navigation**
Header with "Equalizer" and back arrow returning to Dashboard.

---

### Task 12: Frontend Subpages — Gestures, Device Settings & System Settings (Autostart)

**Files:**
- Create: `src/pages/GesturesPage.tsx`
- Create: `src/pages/DeviceSettingsPage.tsx`
- Create: `src/pages/SystemSettingsPage.tsx`
- Create: `src/components/settings/EarTipFitTestModal.tsx`
- Create: `src/components/settings/FindMyBudsModal.tsx`

**Interfaces:**
- Produces:
  - Gestures subpage for touch/pinch configuration.
  - Device settings subpage for In-Ear detection, Ear Tip Fit Test, Find My Buds, and Firmware version.
  - System settings subpage for "Launch at system startup" toggle and "Minimize to tray on close" toggle.

- [ ] **Step 1: Build GesturesPage**
Configure left and right earbud stem gestures.

- [ ] **Step 2: Build DeviceSettingsPage with Fit Test and Find My Buds**
Manage In-Ear wear detection and diagnostic tools.

- [ ] **Step 3: Build SystemSettingsPage with Autostart toggle**
Connect "Launch at startup" toggle to `tauri-plugin-autostart`.

---

### Task 13: System Tray (Earbuds Icon), Menu Bar & Autostart Integration

**Files:**
- Create: `src-tauri/src/tray.rs`
- Create: `src-tauri/src/autostart.rs`
- Create: `src-tauri/icons/tray-earbuds-active.png`
- Create: `src-tauri/icons/tray-earbuds-inactive.png`
- Modify: `src-tauri/src/lib.rs`

**Interfaces:**
- Produces:
  - Persistent system tray icon shaped like earbuds.
  - Grayed-out / outlined when disconnected; illuminated solid white with battery tooltip/badge when active.
  - Tray context menu for fast ANC switching, battery readout, launch at startup toggle, and dashboard opening.
  - Autostart integration via `tauri-plugin-autostart` for Windows and macOS.
  - Window close interception (hides window to tray).

- [ ] **Step 1: Implement dynamic earbud tray icon & context menu in `tray.rs`**
Handle left-click (toggle window visibility) and right-click context menu with live battery status and ANC mode toggles.

- [ ] **Step 2: Integrate `tauri-plugin-autostart`**
Register autostart plugin and expose IPC command for querying and setting autostart state.

- [ ] **Step 3: Implement window close interception**
Configure `tauri.conf.json` and window event handler in `lib.rs` to hide to tray on close.
---

### Task 14: End-to-End Verification & Windows Smoke Testing

**Files:**
- Test: `tests/` and local executable smoke testing

**Interfaces:**
- Produces: Verified production build running cleanly on Windows.

- [ ] **Step 1: Run complete Rust test suite**
Run: `cd src-tauri && cargo test --all-targets`
Expected: All unit and mock integration tests PASS.

- [ ] **Step 2: Run frontend build and linter**
Run: `pnpm build`
Expected: TypeScript and Vite build PASS with zero errors.

- [ ] **Step 3: Run Tauri development build**
Run: `pnpm tauri dev` or `cargo tauri build`
Expected: Desktop window launches, tray icon initializes, mock and live Bluetooth connections operate seamlessly.
