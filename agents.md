# Agent Directives & Engineering Handbook — earX

This file defines the mandatory engineering practices, architecture rules, subagent specializations, and execution invariants for developing the **earX** cross-platform desktop application.

---

## 1. Core Directives & Global Invariants

### 1.1 Secrets & Privacy
* **Strict Rule:** NEVER read, inspect, print, send, or transfer files/data from `~/secrets/` or `~/Secrets/` to models, logs, prompts, or external endpoints.
* **Local-Only Operation:** `earX` is strictly offline. NEVER introduce telemetry, external analytics, remote API calls, or cloud backends.

### 1.2 URL & Routing Style
* **Strict Rule:** NEVER generate URLs with query strings (`?key=value`) in web apps or frontend routing — no exceptions.
* Use clean path segments (e.g. `/device/equalizer`, `/device/gestures`, `/settings/diagnostics`) or local in-memory store states.

### 1.3 Privilege & Script Conventions
* Optional/reusable scripts location: `~/opt/<scripts>` (or `/opt/agents/scripts/`).
* Sudo execution: Use `/opt/agents/scripts/run-sudo.sh <cmd>` for privileged actions without credential exposure.

---

## 2. earX Technical Invariants & Protocol Rules

### 2.1 Bluetooth Transport & Thread Safety
* **Non-Blocking Main Thread:** NEVER perform synchronous or blocking Bluetooth I/O on the Tauri main thread. All RFCOMM/SPP interactions MUST run in asynchronous Tokio background worker tasks.
* **Cross-Platform Conditional Compilation:** Platform-specific code MUST be guarded with `#[cfg(target_os = "windows")]` and `#[cfg(target_os = "macos")]`. Common logic MUST reside behind the `BluetoothTransport` trait.
* **Packet Throttling:** Earbud microcontrollers cannot process rapid bursts of serial commands during initialization. Maintain a minimum $100\text{ms}$ delay between sequential init commands (`init_device`).
* **Sequence ID Wrapping:** Operation / Sequence numbers MUST wrap monotonically from `1` to `250` before resetting to `1`.
* **CRC-16 Modbus Verification:** Every outbound packet MUST compute CRC-16 with polynomial `0xA001` (init `0xFFFF`). Inbound packets with failing CRCs MUST be dropped and logged.

### 2.2 Audio & EQ Math Quirks
* **Custom EQ Float Transformation:** Nothing earbud firmware expects 32-bit float values ($-6.0\text{dB} \dots +6.0\text{dB}$) in reversed byte order with a sign bit inversion on byte 3. Always use the verified `format_float_for_eq` and `from_format_float_for_eq` implementations in the Rust protocol engine.
* **Bass Enhance Scaling:** Enhanced bass levels are passed as `level * 2` (values $0 \dots 5 \to 0 \dots 10$).

---

## 3. Subagent Roles & Delegation Guidelines

When dispatching tasks or executing implementation plans, delegate work according to these specialist domains:

### 3.1 `Rust-Core` (Backend & Protocol Specialist)
* **Scope:**
  * Bluetooth transport drivers (Windows WinRT/Winsock RFCOMM & macOS IOBluetooth bindings).
  * Binary frame serializer, deserializer, CRC16 calculator, and packet streaming pipeline.
  * Device state machine actor (`DeviceManager`) and auto-reconnection loop.
  * Tauri IPC commands and real-time event emitters (`app.emit("device-state-changed", state)`).
* **Key Invariants:** All errors wrapped in structured `thiserror` enums; zero panics in asynchronous loops.

### 3.2 `Frontend-UI` (Design & Interaction Specialist)
* **Scope:**
  * Nothing OS design system components, typography (NDot, Space Grotesk, Roboto Mono), and color tokens.
  * Interactive SVG/Canvas Equalizer curve visualizer and parametric EQ controls.
  * Earbud 3D/2D visualizer with dynamic charging and battery telemetry displays.
  * Gesture customization matrix, quick settings toggles, and ear tip fit test modal.
  * Zustand client-side store synchronized with Tauri backend events.
* **Key Invariants:** Clean URL paths (no query parameters), responsive layout, zero external web requests.

### 3.3 `System-Integration` (OS, Tray & Global Hotkeys Specialist)
* **Scope:**
  * System tray icon management (dynamic battery level badges on Windows Taskbar and macOS Menu Bar).
  * Context menu for rapid ANC toggling and sound profile switching.
  * Global keyboard shortcuts registration and background daemon management.
  * Window lifecycle (minimize-to-tray on close).

### 3.4 `Protocol-Verifier` (Testing & Simulation Specialist)
* **Scope:**
  * Mock Bluetooth transport (`MockBluetoothTransport`) simulating earbud responses for CI/CD and offline development without physical hardware.
  * Comprehensive test suites verifying packet serialization, CRC16 calculation, SKU matching, and float32 EQ conversion.

---

## 4. Code Quality, Standards & Verification

### 4.1 Rust Guidelines
* Strict formatting: `cargo fmt --check`.
* Linter compliance: `cargo clippy -- -D warnings`.
* Logging: Use `tracing` crate with structured logs (`tracing::info!`, `tracing::debug!`, `tracing::error!`). NEVER use raw `println!`.
* Architecture: Maintain clean separation between Transport Layer, Protocol Framing Layer, State Layer, and Tauri IPC Layer.

### 4.2 Frontend Guidelines
* TypeScript Strict Mode: No `any` types. Define complete TypeScript interfaces for all device states, SKU profiles, and packet payloads.
* Styling: Tailwind CSS with custom configuration for Nothing OS colors (`#121212`, `#21201f`, `#d71920`, etc.).
* State Management: Single source of truth in Zustand store, updated exclusively via Tauri event listeners.

### 4.3 Testing & Delivery Checklist
Before delivering any feature:
1. **Unit Tests:** Run `cargo test` on packet framing, CRC, SKU parser, and float converters.
2. **Mock Integration Tests:** Verify end-to-end IPC flow with `MockBluetoothTransport`.
3. **UI Verification:** Ensure the interface adheres to Nothing OS design rules and does not flash or lag during state updates.
4. **Cross-Platform Check:** Verify compilation on target operating systems (`cargo check --target x86_64-pc-windows-msvc` and `cargo check --target aarch64-apple-darwin` / `x86_64-apple-darwin`).
