# earX Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a cross-platform desktop application (Tauri v2 + Rust + React/TypeScript) to monitor and control Nothing and CMF earbuds (specifically CMF Buds Pro 2 / CMF Buds 2 Plus, Nothing Ear, Nothing Ear (2), CMF Buds, etc.) over Bluetooth SPP/RFCOMM on Windows and macOS.

**Architecture:** A lightweight Rust backend handles raw binary packet framing, CRC16 Modbus calculations, platform-specific Bluetooth RFCOMM connections, and state management actor loops. Tauri v2 IPC exposes commands and streams state updates to a React 18 frontend with an authentic Nothing OS industrial design system.

**Tech Stack:** Tauri v2, Rust 1.78+ (`tokio`, `serialport`, `windows`, `thiserror`, `tracing`, `serde`), React 18, Vite, TypeScript, Tailwind CSS, Lucide icons, Zustand.

## Global Constraints

- **Cross-Platform:** Pure Rust implementation with Windows (`windows-rs` / WinRT RFCOMM & serial COM fallback) and macOS (`IOBluetooth` FFI & POSIX TTY fallback).
- **Non-Blocking:** All Bluetooth I/O in Tokio worker tasks; zero blocking operations on the Tauri main thread.
- **Protocol Precision:** CRC-16 Modbus (`0xA001`, init `0xFFFF`), sequence wrapping $1 \dots 250$, minimum $100\text{ms}$ delay between sequential init commands.
- **URL Style:** Zero query strings (`?key=value`) — clean paths or in-memory routing only.
- **Security:** Offline operation only, zero external web requests, zero access to `~/secrets/`.

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

### Task 10: Frontend Main Dashboard — Device Visualizer & Noise Control

**Files:**
- Create: `src/components/dashboard/DeviceVisualizer.tsx`
- Create: `src/components/dashboard/BatteryBadge.tsx`
- Create: `src/components/dashboard/NoiseControlSelector.tsx`
- Create: `src/pages/DashboardPage.tsx`

**Interfaces:**
- Produces: Main screen displaying earbud charging graphic, left/right/case battery %, and ANC dial (Noise Cancellation / Transparency / Off with intensity slider).

- [ ] **Step 1: Build `BatteryBadge` component**
Render battery percentage with dynamic charging bolt icon and color state (green / amber / red).

- [ ] **Step 2: Build `DeviceVisualizer` component**
Render Left, Case, and Right earbud assets with subtle charging glow effects.

- [ ] **Step 3: Build `NoiseControlSelector` component**
Render 3-way toggle (Noise Cancellation, Transparency, Off) with level sub-selector (High, Mid, Low, Adaptive).

- [ ] **Step 4: Assemble `DashboardPage`**
Connect visualizer and noise controls to `useDeviceStore`.

---

### Task 11: Frontend Equalizer Hub — 3-Band Wave & Ultra Bass

**Files:**
- Create: `src/components/equalizer/EqPresetSelector.tsx`
- Create: `src/components/equalizer/WaveEqualizerCanvas.tsx`
- Create: `src/components/equalizer/UltraBassSlider.tsx`
- Create: `src/pages/EqualizerPage.tsx`

**Interfaces:**
- Produces: Interactive audio customization screen with smooth SVG EQ curve manipulation and Ultra Bass control.

- [ ] **Step 1: Build `EqPresetSelector`**
Presets for Balanced, More Bass, More Treble, Voice, and Custom.

- [ ] **Step 2: Build `WaveEqualizerCanvas`**
Interactive 3-band curve visualizer (Bass, Mid, Treble) updating in real-time.

- [ ] **Step 3: Build `UltraBassSlider`**
$0 \dots 5$ discrete level slider with instant tactile visual feedback.

- [ ] **Step 4: Assemble `EqualizerPage`**
Bind EQ controls to Tauri IPC commands.

---

### Task 12: Frontend Gestures & Quick Settings Hub

**Files:**
- Create: `src/components/gestures/GestureStemSelector.tsx`
- Create: `src/components/gestures/GestureActionRow.tsx`
- Create: `src/components/settings/QuickSettingToggle.tsx`
- Create: `src/components/settings/EarTipFitTestModal.tsx`
- Create: `src/components/settings/FindMyBudsControl.tsx`
- Create: `src/pages/GesturesPage.tsx`
- Create: `src/pages/SettingsPage.tsx`

**Interfaces:**
- Produces: Complete gesture mapping screen and settings hub with In-Ear detection, Low Latency, Ringing, and Fit Test.

- [ ] **Step 1: Build Gesture stem and action configuration**
Configure Double Pinch, Triple Pinch, Pinch & Hold, Double Pinch & Hold for Left and Right buds.

- [ ] **Step 2: Build Quick Settings toggles and Find My Buds**
Toggles for In-Ear Wear Detection, Low Latency Mode, and Left/Right audio beacon ringing.

- [ ] **Step 3: Build Ear Tip Fit Test modal**
Acoustic test wizard with seal analysis result feedback.

---

### Task 13: System Tray & Desktop Integration

**Files:**
- Create: `src-tauri/src/tray.rs`
- Modify: `src-tauri/src/lib.rs`

**Interfaces:**
- Produces: Resident background system tray icon with live battery display, quick ANC menu switcher, and minimize-to-tray window lifecycle.

- [ ] **Step 1: Configure Tauri System Tray**
Create tray icon with dynamic tooltip showing battery status.

- [ ] **Step 2: Add Tray context menu**
Menu items for Quick ANC toggle, Open Dashboard, and Quit.

- [ ] **Step 3: Implement Minimize-to-Tray on window close**
Intercept window close event and hide window to keep background monitoring alive.

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
