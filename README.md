<div align="center">

# earX

**Native Cross-Platform Desktop Controller for Nothing & CMF Earbuds**

[![CI](https://github.com/aaxyat/earX/actions/workflows/ci.yml/badge.svg)](https://github.com/aaxyat/earX/actions/workflows/ci.yml)
[![Release](https://github.com/aaxyat/earX/actions/workflows/release.yml/badge.svg)](https://github.com/aaxyat/earX/actions/workflows/release.yml)
[![Tauri v2](https://img.shields.io/badge/Tauri-v2.2-24C8D8?style=flat&logo=tauri&logoColor=white)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Rust-1.78+-orange?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![React](https://img.shields.io/badge/React-18.3-61DAFB?style=flat&logo=react&logoColor=black)](https://react.dev)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS-blue?style=flat)](https://github.com/aaxyat/earX)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL%203.0-red.svg)](LICENSE)

<br/>

<p align="center">
  A native, ultra-lightweight desktop application crafted with the authentic <b>Nothing OS / Nothing X</b> industrial design language. Control Active Noise Cancellation, Ultra Bass, interactive Equalizers, and stem gestures directly from Windows and macOS with an always-running system tray resident.
</p>

</div>

---

## ✨ Features

- **🎨 Pixel-Perfect Nothing OS Aesthetic:** True black (`#000000`) canvas with 2-column bento card grid, signature Nothing red accents, serif headlines, and dot-matrix typography.
- **⚡ Real-Time Battery Telemetry:** Individual progress bars and charging indicators for Left earbud, Right earbud, and Charging Case (`— L 95%`, `⚡ — C 40%`, `— R 90%`).
- **🎧 Active Noise Cancellation Controls:** 3 circular mode dials (Noise Cancellation, Transparency, Off) with a 4-step segmented pill selector (`Low` | `Mid` | `High` | `Adaptive`).
- **🎛️ Interactive Wave Equalizer:** Smooth bezier curve canvas with draggable Bass, Mid, and Treble nodes ($-6.0\text{dB} \dots +6.0\text{dB}$) plus sound profile presets (`Balanced`, `Rock`, `Pop`, `Voice`, etc.).
- **🔊 Ultra Bass System:** Dedicated toggle with a 5-bar Nothing red level meter (`|||||`) offering discrete $1 \dots 5$ bass enhancement levels.
- **👆 Custom Stem Gestures:** Configure double tap, triple tap, tap & hold, and double tap & hold for left and right earbuds independently.
- **🔍 Diagnostic Tools & Fit Test:** Acoustic seal Ear Tip Fit Test wizard and high-pitched Find My Earbuds locator beacon.
- **💻 Always-Running System Tray & Menu Bar:** Persistent background resident with an earbud tray icon, live battery tooltip, quick ANC switcher, and minimize-to-tray on close.
- **🚀 Autostart at System Launch:** Integrated launch-on-boot configuration for Windows (Registry) and macOS (LaunchAgent).
- **🔒 Pure Local & Offline:** Direct Bluetooth Classic RFCOMM / Serial Port Profile (SPP) communication with zero cloud dependencies, telemetry, or external network requests.

---

## 📱 Compatibility Matrix

| Hardware Device | Code Name | Hardware Base ID | Supported Features |
|---|---|---|---|
| **CMF Buds Pro 2 / Buds 2 Plus** | *espeon* | `B172` | Smart ANC (4 levels + Trans), Ultra Bass (1-5), Listening Modes, In-Ear, Low Latency, Fit Test, Gestures |
| **Nothing Ear (2024)** | *entei* | `B171` | Smart ANC (45dB), Advanced 8-band EQ + 3-band EQ, Ultra Bass, In-Ear, Low Latency, Fit Test |
| **Nothing Ear (2)** | *two* | `B155` | Personalized ANC, Advanced EQ, In-Ear, Low Latency, Dual Conn, Fit Test |
| **Nothing Ear (1)** | *one* | `B181` | ANC (Light/Max/Trans), 3-band EQ, In-Ear, Case LED RGB Control, Ringing |
| **Nothing Ear (a)** | *cleffa* | `B162` | Smart ANC (45dB), Ultra Bass, 3-band EQ, In-Ear, Low Latency, Fit Test |
| **Nothing Ear (stick)** | *sticks* | `B157` | 3-band EQ, In-Ear, Low Latency, Bass Lock |
| **Nothing Ear (open)** | *flaaffy* | `B174` | Open-ear audio, Bass Enhance, 3-band EQ, Low Latency |
| **CMF Buds** | *donphan* | `B168` | ANC (3 levels + Trans), Ultra Bass (1-5), Listening Modes, In-Ear, Low Latency, Find Buds |
| **CMF Buds Pro** | *corsola* | `B163` | ANC (High/Mid/Low/Trans), 3-band EQ, In-Ear, Low Latency, Find Buds |
| **CMF Neckband Pro** | *crobat* | `B164` | ANC (50dB Hybrid), Ultra Bass, In-Ear, Low Latency |

---

## 🛠️ Architecture & Tech Stack

```
earX
├── Frontend (React 18 + Vite + TypeScript + Tailwind CSS)
│   ├── Nothing OS Design Tokens & NDot / SpaceGrotesk typography
│   ├── Zustand Store with optimistic updates & real-time event sync
│   └── SVG Interactive Wave Equalizer & Bento Card Grid
│
└── Backend (Tauri v2 + Rust 1.78+)
    ├── Protocol Engine (Binary packet framing + CRC-16 Modbus)
    ├── Custom EQ IEEE-754 Float32 Byte Transformer
    ├── SKU & Hardware Identification Engine
    ├── Platform Bluetooth Transport (Windows RFCOMM & macOS IOBluetooth)
    └── State Machine Actor (DeviceManager with 100ms command pacing)
```

---

## 📦 Installation & Releases

Download pre-built installers for your operating system from the **[Releases](https://github.com/aaxyat/earX/releases)** page:
- **Windows:** `.msi` installer or standalone `.exe`
- **macOS:** `.dmg` installer or `.app` bundle (Universal for Apple Silicon & Intel)

---

## 💻 Building from Source

### Prerequisites

#### Windows:
1. **Rust:** Install via [rustup.rs](https://rustup.rs) (`stable-x86_64-pc-windows-msvc`).
2. **Visual Studio C++ Build Tools:** Install Visual Studio 2022 Build Tools with the **Desktop development with C++** workload.
3. **Node.js & pnpm:** Node.js 18+ and `pnpm` (`npm install -g pnpm`).

#### macOS:
1. **Rust:** Install via [rustup.rs](https://rustup.rs) (`stable`).
2. **Xcode Command Line Tools:** `xcode-select --install`.
3. **Node.js & pnpm:** Node.js 18+ and `pnpm`.

---

### Development Setup

```bash
# 1. Clone the repository
git clone https://github.com/aaxyat/earX.git
cd earX

# 2. Install frontend dependencies
pnpm install

# 3. Run unit & integration tests
cd src-tauri && cargo test --all-targets && cd ..

# 4. Start live desktop development server
pnpm tauri dev
```

### Production Build

```bash
# Build standalone desktop release installer
pnpm tauri build
```
The compiled binaries will be placed in `src-tauri/target/release/bundle/`.

---

## 📄 License

This project is published under the **GNU General Public License v3.0 (GPL-3.0)**. See [`LICENSE`](LICENSE) for details.

*Disclaimer:* `earX` is an independent open-source project developed for the community and is not officially affiliated with, endorsed by, or sponsored by Nothing Technology Limited.

---

<div align="center">
  <sub>Crafted for the Nothing & CMF Community</sub>
</div>
