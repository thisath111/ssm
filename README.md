# Smart System Manager (`ssm`)

> High-performance, AI-driven Windows system optimizer written in 100% Native Rust.  
> Runs silently as a background service — zero UI, zero bloat, maximum impact.

[![Build](https://github.com/thisath111/ssm/actions/workflows/release.yml/badge.svg)](https://github.com/thisath111/ssm/actions/workflows/release.yml)
[![Tests](https://github.com/thisath111/ssm/actions/workflows/audit.yml/badge.svg)](https://github.com/thisath111/ssm/actions/workflows/audit.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows_10_%7C_11_(x64)-0078D6)](https://microsoft.com/windows)
[![Release](https://img.shields.io/github/v/release/thisath111/ssm)](https://github.com/thisath111/ssm/releases/latest)

---

## What is ssm?

`ssm` is a background Windows optimization daemon that applies deep kernel-level tuning that Windows never does out of the box. A **Pure-Rust AI engine** classifies what you are doing in real time — Gaming, Coding, Video Editing, or Idle — then dynamically adjusts CPU priority, GPU scheduling, RAM pressure, and network quality. Automatically. No configuration needed.

Unlike bloatware "optimizer" apps, `ssm` has no GUI, no ads, no telemetry, and no dependencies. Open-source, written entirely in Rust, consuming less than `0.01% CPU` and `~6 MB RAM`.

---

## Quick Install

Open **PowerShell as Administrator** and run:

```powershell
irm https://raw.githubusercontent.com/thisath111/ssm/main/install.ps1 | iex
```

This will:
1. Download the latest optimized binary to `C:\ssm\ssm.exe`
2. Add `C:\ssm` to your system `PATH`
3. Register `ssm` as a Windows Native Service (auto-starts on every boot)
4. Start the background optimization daemon immediately

To uninstall completely:
```powershell
ssm uninstall
```

---

## Manual Installation

1. Download **`ssm.exe`** from [Releases](https://github.com/thisath111/ssm/releases/latest)
2. Place it in `C:\ssm\` and add that folder to your `PATH`
3. Register as a Windows Service:
   ```powershell
   ssm service install
   ```

---

## Features

### ⚡ Hardware-Adaptive Kernel Tuning
- **Adaptive Timer Resolution** — Locks system timer via `NtSetTimerResolution` at 1.0ms / 0.75ms / 0.5ms based on detected hardware tier (Low / Mid / High-end)
- **Interrupt Affinity Optimization** — Dynamically routes GPU and Network PCI interrupts directly to Performance Cores (`p_core_mask`), eliminating Core 0 DPC latency bottlenecks
- **CPU C-State & Idle Suppression** — Suppresses deep CPU idle C-states during gaming/boost workloads via `PowerWriteACValueIndex` to eliminate micro-stutters
- **MMCSS Thread Scheduling Boost** — Registers the engine thread with the Windows Multimedia Class Scheduler Service (`AvSetMmThreadCharacteristicsW`) for hard real-time priority
- **Hardware Tier Detection** — Auto-detects CPU cores and total RAM at startup and tunes every threshold, trim level, and I/O policy to match your specific PC
- **P-Core / E-Core Affinity** — Detects CPU topology and pins background processes to Efficiency Cores, reserving Performance Cores for the foreground
- **App Launch Accelerator** — Boosts newly spawned processes to `HIGH_PRIORITY_CLASS` for 3 seconds for instant responsiveness

### 🔄 Automatic Updates & Self-Healing Microkernel
- **Zero-Downtime Supervised Engine** — Runs optimization routines inside a panic-isolated supervisor loop (`catch_unwind`) with automatic zero-downtime recovery
- **Configuration Hot-Reload** — Daemon dynamically applies updated configuration settings without requiring a service restart
- **Weekly Auto-Update Check** — Automatically checks GitHub releases weekly in the background and applies in-place atomic updates (`self_replace`)
- **CLI Update Management** — Manually check, install, enable, or disable automatic background updates with simple commands

### 🧠 Zero-Hardcode AI Engine
- **Behavioral Process Intent Classifier** — Identifies Gaming, Creative Workstation, Developer, or Typing Tool workloads purely from behavioral signals (RAM usage, CPU %, thread count, window ownership) — no process name lists
- **Dynamic Window Ownership Detection** — Uses Win32 `EnumWindows` to discover every interactive process, tray app, input hook, and IME (e.g. Helakuru, Keyman, Wijesekara) at runtime — nothing hardcoded
- **Predictive Memory Forecaster** — Tracks RAM velocity to forecast pressure spikes 5 seconds ahead and flushes standby cache *before* stuttering occurs
- **Workload State Machine** — Transitions between `UltraGaming`, `CreatorBoost`, `StandardInteractive`, and `PowerSaverIdle` with hardware-adaptive emergency thresholds

### 🚀 Deep Windows Subsystem Optimization
- **Global Timer Resolution Request (Win 11 Fix)** — Enforces global 0.5ms timer resolution across all processes via kernel Session Manager
- **Win32 Priority Separation (0x26)** — Enhances foreground thread responsiveness by prioritizing interactive foreground quantum
- **USB Latency Boost** — Disables USB Selective Suspend to minimize mouse and keyboard input lag
- **DWM DirectFlip Zero-Lag** — Strips Desktop Window Manager animation delays and routes frames directly to GPU overlay scan-out
- **Kernel Large Pages (2MB HugePages)** — Minimizes CPU TLB cache misses for compilers and game engines
- **GPU & Network MSI-X Interrupts** — Switches adapters to Message Signaled Interrupts via PCI Registry, eliminating DPC latency spikes
- **NVMe Queue Depth Tuning** — Maximizes NVMe parallel I/O queues and NTFS MFT cache allocation
- **TCP NoDelay & QoS DSCP** — Disables Nagle algorithm and enforces QoS packet prioritization for lower network latency

### 🛡️ Stability & Anti-Hang Shield
- **Typing Tool Immunity** — Any input method or typing software is automatically detected via window/hook ownership and fully protected from throttling or suspension
- **Explorer Auto-Rescue** — Monitors `explorer.exe` via `IsHungAppWindow` and auto-rescues it within seconds if an I/O deadlock freezes your taskbar
- **Behavioral Background Throttling** — Identifies background disk hogs by behavior (low CPU + sleeping state) rather than by name, and deprioritizes them dynamically
- **Fast Shutdown** — Reduces service shutdown timeouts and enables `AutoEndTasks` for instant shutdowns
- **Handle Leak Auditor** — Monitors processes holding excessive OS handles and deprioritizes them automatically

---

## CLI Reference

```powershell
ssm stats              # Live hardware telemetry, AI app intent & kernel stats
ssm boost              # One-click performance boost (Timer, Large Pages, DWM, GPU, NVMe)
ssm clean              # Purge Windows Standby RAM + clean temporary files
ssm tune               # Apply low-latency registry tweaks (MSI-X, TCP NoDelay, Fast Shutdown)
ssm update             # Check and install the latest ssm update automatically
ssm update --check     # Check if a new version is available without installing
ssm update --enable    # Enable automatic weekly background updates
ssm update --disable   # Disable automatic weekly background updates
ssm daemon             # Run the optimization daemon in foreground (console mode)
ssm service install    # Register as a Windows Service with boot autostart
ssm service uninstall  # Unregister the Windows Service
ssm uninstall          # Completely remove ssm (reverts tweaks, removes service & PATH)
```

---

## System Requirements

| | |
|---|---|
| **OS** | Windows 10 / 11 (64-bit) |
| **Architecture** | x86-64 (AMD64) |
| **Privileges** | Administrator (required for kernel-level tuning) |
| **Runtime** | None — fully self-contained binary |

---

## Building from Source

**Requirements:** [Rust 1.75+](https://rustup.rs/) · Windows 10/11 64-bit · MSVC Build Tools

```powershell
git clone https://github.com/thisath111/ssm.git
cd ssm
cargo build --release   # → target/release/ssm.exe
cargo test              # Run all unit tests
```

---

## License

Distributed under the **MIT License**. See [`LICENSE`](LICENSE) for details.
