# Smart System Manager (`ssm`)

> High-performance, AI-driven Windows system optimizer written in 100% Native Rust.  
> Runs silently as a background service — zero UI, zero bloat, maximum impact.

[![Build](https://github.com/thisath111/ssm/actions/workflows/release.yml/badge.svg)](https://github.com/thisath111/ssm/actions)
[![Tests](https://github.com/thisath111/ssm/actions/workflows/audit.yml/badge.svg)](https://github.com/thisath111/ssm/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows_10_%7C_11_(x64)-0078D6)](https://microsoft.com/windows)
[![Version](https://img.shields.io/badge/version-1.0.0-blue)](https://github.com/thisath111/ssm/releases/latest)

---

## What is ssm?

`ssm` is a background Windows optimization daemon that applies deep kernel-level tuning techniques that Windows itself never does out of the box. It uses a **Pure-Rust micro-AI engine** to classify what you are doing in real time (Gaming, Coding, Video Editing, Idle), then dynamically adjusts CPU priority, GPU scheduling, RAM pressure, and network quality — all automatically.

Unlike bloatware "optimizer" apps, `ssm` has no GUI, no ads, no background telemetry, and no BS. It is open-source, written entirely in Rust, and consumes less than `0.01% CPU` and `~6 MB RAM`.

---

## Quick Install (1 Line)

Open **PowerShell as Administrator** and run:

```powershell
irm https://raw.githubusercontent.com/thisath111/ssm/main/install.ps1 | iex
```

This will:
1. Download the latest optimized binary to `C:\ssm\ssm.exe`
2. Add `C:\ssm` to your system `PATH`
3. Register `ssm` as a Windows Native Service (auto-starts on every boot)
4. Start the background optimization daemon immediately

---

## Manual Installation (Without Script)

If you prefer to install manually:

1. Download **`ssm.exe`** from [Releases](https://github.com/thisath111/ssm/releases/latest)
2. Create a folder `C:\ssm\` and place `ssm.exe` inside it
3. Add `C:\ssm` to your `PATH` (run once in Admin PowerShell):
   ```powershell
   [Environment]::SetEnvironmentVariable("Path", "$([Environment]::GetEnvironmentVariable('Path','Machine'));C:\ssm", "Machine")
   ```
4. Register as a Windows Service and enable boot autostart:
   ```powershell
   ssm service install
   ```
5. Restart your terminal — `ssm` is now running in the background

To stop and remove at any time:
```powershell
ssm uninstall
```

---

## Features

### ⚡ Sub-Millisecond Kernel Tuning
- **0.5ms Win32 Timer Resolution** — Locks system timer via `NtSetTimerResolution(5000)` for immediate mouse and keyboard responsiveness
- **P-Core / E-Core Affinity** — Detects your CPU topology and pins background hogs to Efficiency Cores, freeing Performance Cores for what matters
- **App Launch Accelerator** — Detects new processes and temporarily boosts them to `HIGH_PRIORITY_CLASS` for 3 seconds for instant responsiveness

### 🧠 AI Process Scheduler
- **Process Intent Classifier** — Identifies whether your foreground app is Gaming, Creative (Video/3D), Developer/Compiler, or General Productivity and tunes resources accordingly
- **Predictive Memory Forecaster** — Tracks RAM velocity to forecast pressure spikes 5 seconds ahead and flushes standby cache *before* stuttering occurs
- **Dynamic Workload State Machine** — Switches between `UltraGaming`, `CreatorBoost`, `StandardInteractive`, and `PowerSaverIdle` modes automatically

### 🚀 Deep Windows Subsystem Optimization
- **DWM DirectFlip Zero-Lag** — Strips Desktop Window Manager animation delays and routes frames directly to GPU overlay scan-out
- **Kernel Large Pages (2MB HugePages)** — Grants `SeLockMemoryPrivilege` to minimize CPU TLB cache misses for compilers and game engines
- **GPU & Network MSI-X Interrupts** — Switches GPU and Network adapters to Message Signaled Interrupts via PCI Registry, eliminating DPC latency spikes
- **NVMe Queue Depth Tuning** — Maximizes NVMe parallel I/O request queues and NTFS MFT cache allocation
- **TCP NoDelay & QoS DSCP** — Disables Nagle algorithm and enforces QoS CS1 packet prioritization for lower network latency

### 🛡️ Stability & Anti-Hang Shield
- **Explorer Auto-Rescue** — Monitors `explorer.exe` via `IsHungAppWindow`. If a damaged USB drive freezes your taskbar, `ssm` detects and restarts it within seconds — no Task Manager needed
- **Network Burst Throttling** — Permanently locks notorious Windows background services (`TiWorker.exe`, `compattelrunner.exe`, `wermgr.exe`, `mrt.exe`) to `IDLE_PRIORITY_CLASS`, eliminating the "internet-on" PC freeze
- **Fast Shutdown** — Reduces `WaitToKillServiceTimeout` and `WaitToKillAppTimeout` to 2 seconds and enables `AutoEndTasks` for instant shutdowns
- **Core Immunity Matrix** — Ensures `csrss.exe`, `lsass.exe`, `dwm.exe`, and other critical system processes are never touched
- **Handle Leak Auditor** — Continuously monitors for processes holding over 10,000 OS handles (a common sign of resource leaks) and deprioritizes them

---

## CLI Reference

```powershell
# Show live hardware telemetry, AI-detected app intent & kernel stats
ssm stats

# Apply an immediate one-click performance boost
# (CPU Unparking, 0.5ms Timer, Large Pages, DWM DirectFlip, GPU Boost, NVMe Tuning)
ssm boost

# Purge Windows Standby RAM list and clean temporary storage files
ssm clean

# Apply low-latency registry tweaks
# (MSI-X Interrupts, Fast Shutdown, TCP NoDelay, Telemetry Off)
ssm tune

# Run the optimization daemon in the foreground (console mode)
ssm daemon

# Register ssm as a Windows Service with boot autostart
ssm service install

# Unregister and stop the Windows Service
ssm service uninstall

# Completely remove ssm from your system
# (Reverts tweaks, removes service, removes PATH entry, deletes C:\ssm)
ssm uninstall
```

---

## Building from Source

**Requirements:**
- [Rust 1.75+](https://rustup.rs/) (install via rustup)
- Windows 10 or 11 (64-bit)
- Microsoft C++ Build Tools (via Visual Studio Installer)

```powershell
# Clone the repository
git clone https://github.com/thisath111/ssm.git
cd ssm

# Build an optimized release binary
cargo build --release

# The output binary will be at:
# ./target/release/ssm.exe

# Run all unit tests
cargo test -- --nocapture
```

> The official release binary is built on GitHub Actions with `target-cpu=native` and `opt-level=3` for maximum runtime performance.

---

## License

Distributed under the **MIT License**. See [`LICENSE`](LICENSE) for details.
