<div align="center">

# ⚡ Smart System Manager (`ssm`)

**Ultra-Low Latency Windows Optimization Daemon & Autonomous AI Microkernel**  
*Engineered in 100% Native Rust · Zero Bloat · Zero Telemetry · Sub-Millisecond Kernel Tuning*

[![Build Status](https://img.shields.io/github/actions/workflow/status/thisath111/ssm/release.yml?branch=main&style=for-the-badge&logo=github&logoColor=white&label=Build)](https://github.com/thisath111/ssm/actions)
[![Tests](https://img.shields.io/badge/Tests-13%20Passing-brightgreen?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/thisath111/ssm/actions)
[![Latest Release](https://img.shields.io/github/v/release/thisath111/ssm?style=for-the-badge&color=blue&logo=windows&logoColor=white)](https://github.com/thisath111/ssm/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow?style=for-the-badge)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011%20(x64)-0078D6?style=for-the-badge&logo=windows11&logoColor=white)](https://microsoft.com/windows)

<p align="center">
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-manual-installation">Manual Install</a> •
  <a href="#-building-from-source">Build from Source</a> •
  <a href="#-command-reference">CLI Reference</a> •
  <a href="#-features--architecture">Features</a> •
  <a href="#-troubleshooting--faq">FAQ</a>
</p>

---

</div>

## 📌 Overview

**`ssm`** is a lightweight, background Windows performance optimizer that enforces deep, hardware-level kernel tuning that Windows never does out of the box. 

Powered by a **Pure-Rust sub-microsecond AI engine**, `ssm` continuously analyzes active foreground workloads (Gaming, Video/Audio Creation, Development, or Idle) and dynamically adapts CPU scheduling, GPU queues, RAM pressure, and network interrupts in real time.

```
┌────────────────────────────────────────────────────────────────────────┐
│  ⚡ Hardware-Adaptive Windows Optimization Microkernel                 │
├────────────────────────────────────────────────────────────────────────┤
│  • Memory Footprint : ~6.0 MB RAM       • CPU Overhead : < 0.01%       │
│  • Timer Resolution : Locked @ 0.5ms    • User Interface: 100% Silent  │
│  • System Privileges: Win32 NT Kernel   • Architecture : Native x86-64 │
└────────────────────────────────────────────────────────────────────────┘
```

> [!TIP]
> **Zero Bloatware Guarantee:** `ssm` contains **no GUI, no Electron, no advertisements, no webviews, and no analytics**. It runs entirely as a native Windows service.

---

## ⚡ Quick Start

### 1-Line Automated Installation (PowerShell)

Open **PowerShell as Administrator** and execute:

```powershell
irm https://raw.githubusercontent.com/thisath111/ssm/main/install.ps1 | iex
```

#### What happens automatically:
1. Downloads pre-compiled `ssm.exe` to `C:\ssm\`
2. Permanently registers `C:\ssm` in your Windows system `PATH`
3. Registers `SmartSystemManager` as an autostarting **Windows Native Service**
4. Launches the optimization daemon immediately in the background

---

## 🛠️ Manual Installation

Prefer manual control without running scripts? Follow this simple 4-step setup:

### Step 1: Download Binary
Download **`ssm.exe`** directly from [GitHub Releases](https://github.com/thisath111/ssm/releases/latest).

### Step 2: Place in System Folder
Create `C:\ssm` and place `ssm.exe` inside:
```powershell
New-Item -ItemType Directory -Force -Path "C:\ssm"
Move-Item -Path "$HOME\Downloads\ssm.exe" -Destination "C:\ssm\ssm.exe"
```

### Step 3: Add to System PATH
```powershell
[Environment]::SetEnvironmentVariable(
    "Path",
    [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::Machine) + ";C:\ssm",
    [EnvironmentVariableTarget]::Machine
)
```

### Step 4: Register & Start Windows Service
```powershell
ssm service install
```

Verify everything is working:
```powershell
ssm stats
```

---

## 🦀 Building from Source

Build `ssm` directly from source code with the Rust compiler.

### Prerequisites
- **Rust Toolchain (1.75+)**: [https://rustup.rs/](https://rustup.rs/)
- **Visual Studio MSVC C++ Build Tools**: Required for Windows native linking
- **Git**: [https://git-scm.com/](https://git-scm.com/)

```powershell
# 1. Clone the repository
git clone https://github.com/thisath111/ssm.git
cd ssm

# 2. Run automated test suite (13/13 passing)
cargo test

# 3. Compile optimized release binary
cargo build --release

# 4. Install compiled binary (C:\ssm\ssm.exe)
New-Item -ItemType Directory -Force -Path "C:\ssm"
Copy-Item ".\target\release\ssm.exe" "C:\ssm\ssm.exe"
ssm service install
```

---

## 💻 Command Reference

All CLI commands can be run directly from any PowerShell or Command Prompt window (Admin required for hardware tuning).

```powershell
ssm [COMMAND] [OPTIONS]
```

| Command | Action / Purpose | Example |
|---|---|---|
| `ssm stats` | Live hardware telemetry, AI active workload intent & timer latency | `ssm stats` |
| `ssm boost` | Instant one-click performance boost (Cores, 0.5ms Timer, DWM, Large Pages) | `ssm boost` |
| `ssm clean` | Purges Windows Standby memory cache and cleans temporary disk junk | `ssm clean` |
| `ssm tune` | Applies permanent low-latency registry tweaks (MSI-X, TCP NoDelay, Fast Shutdown) | `ssm tune` |
| `ssm update` | Checks GitHub for updates and installs the latest version automatically | `ssm update` |
| `ssm update --check` | Checks if a newer version exists without downloading | `ssm update --check` |
| `ssm update --enable` | Enables weekly background auto-updates *(Enabled by default)* | `ssm update --enable` |
| `ssm update --disable`| Disables weekly background auto-updates | `ssm update --disable` |
| `ssm daemon` | Runs the optimization daemon in foreground console mode | `ssm daemon` |
| `ssm service install` | Registers and starts `ssm` as a boot-autostart Windows Service | `ssm service install` |
| `ssm service uninstall`| Stops and deletes the Windows Service | `ssm service uninstall` |
| `ssm uninstall` | Completely reverts all tweaks, deletes service, and removes `PATH` | `ssm uninstall` |

---

## 🌟 Features & Architecture

```
                               ┌────────────────────────┐
                               │   Active Applications  │
                               └───────────┬────────────┘
                                           │ (Behavioral Signals)
                                           ▼
┌────────────────────────────────────────────────────────────────────────┐
│                       Pure-Rust AI Engine                              │
│  ┌───────────────────────┐  ┌──────────────────────┐  ┌─────────────┐  │
│  │ Intent Classifier     │  │ Standby Forecaster   │  │ State Engine│  │
│  └───────────────────────┘  └──────────────────────┘  └─────────────┘  │
└──────────────────────────────────┬─────────────────────────────────────┘
                                   │
       ┌───────────────────────────┼───────────────────────────┐
       ▼                           ▼                           ▼
┌──────────────┐            ┌──────────────┐            ┌──────────────┐
│  CPU & Cores │            │  GPU & DWM   │            │ RAM & I/O    │
│ • P/E Affinity            │ • DirectFlip │            │ • Standby    │
│ • C-State Off│            │ • MSI-X Mode │            │   Purge      │
│ • MMCSS Boost│            │ • 0.5ms Clock│            │ • NVMe Depth │
└──────────────┘            └──────────────┘            └──────────────┘
```

### ⚡ 1. Hardware-Adaptive Kernel Tuning
- **0.5ms Win32 Timer Resolution:** Locks system timer via `NtSetTimerResolution` to 0.5ms / 0.75ms / 1.0ms based on hardware tier.
- **Interrupt Affinity Optimization:** Dynamically routes GPU and Network PCI interrupts directly to Performance Cores (`p_core_mask`), preventing Core 0 DPC latency bottlenecks.
- **CPU C-State & Idle Suppression:** Suppresses deep CPU idle states during high-performance workloads via native power index calls to eliminate micro-stuttering.
- **MMCSS Thread Scheduling Boost:** Registers the engine thread with the Windows Multimedia Class Scheduler Service (`AvSetMmThreadCharacteristicsW`) for hard real-time execution priority.
- **P-Core / E-Core Affinity:** Detects CPU topology and pins background processes to Efficiency Cores, reserving Performance Cores for foreground tasks.

### 🧠 2. Zero-Hardcode AI Engine & Behavioral Heuristics
- **Behavioral Process Intent Classifier:** Identifies Gaming, Video/Audio Production, Development, or Typing Tool workloads purely from behavioral signals (RAM, CPU, thread count, window ownership) — no hardcoded process lists.
- **Dynamic Window Ownership Discovery:** Uses Win32 `EnumWindows` to detect interactive processes, tray apps, and input tools (Helakuru, Keyman, Wijesekara) and protects them from throttling.
- **Predictive Standby Memory Forecaster:** Computes RAM velocity momentum ($\frac{dM}{dt}$) to forecast memory pressure spikes 5 seconds ahead and pre-emptively purges standby cache before stutter occurs.

### 🚀 3. Deep Windows Subsystem Optimizations
- **Global Timer Resolution (Win 11 Fix):** Enforces global 0.5ms timer resolution across all processes via kernel Session Manager.
- **Win32 Priority Separation (`0x26`):** Enhances foreground thread responsiveness by prioritizing interactive foreground quantum.
- **DWM DirectFlip Zero-Lag:** Bypasses Desktop Window Manager animation delays and routes frames directly to GPU scan-out.
- **Kernel Large Pages (2MB HugePages):** Unlocks `SeLockMemoryPrivilege` for Large Page memory allocations, reducing CPU TLB cache misses.
- **GPU & Network MSI-X Interrupts:** Switches adapters to Message Signaled Interrupts via PCI Registry to eliminate DPC latency spikes.
- **TCP NoDelay & QoS DSCP:** Disables Nagle algorithm and prioritizes latency-critical packets.

### 🛡️ 4. Stability & Anti-Hang Shield
- **Explorer Auto-Rescue:** Continuously monitors `explorer.exe` message loops via `IsHungAppWindow` and auto-rescues frozen taskbars within seconds.
- **Zero-Downtime Supervised Engine:** Runs all daemon routines in panic-isolated supervisor loops (`catch_unwind`) with automatic recovery.
- **Background Disk Hog Throttling:** Automatically deprioritizes background disk hogs to `IDLE_PRIORITY_CLASS` and lowest I/O priority.
- **Fast Shutdown:** Reduces service shutdown timeouts to 2 seconds and enables `AutoEndTasks`.

---

## ⚙️ Configuration & Logging

<details>
<summary><b>Click to expand Configuration Details (<code>config.json</code>)</b></summary>

The configuration file is automatically created in `C:\ssm\config.json`. The daemon hot-reloads this file every 10 seconds without needing a restart:

```json
{
  "enable_cpu_affinity": true,
  "enable_timer_resolution": true,
  "enable_standby_purging": true,
  "auto_update_enabled": true,
  "last_update_check_unix": 0,
  "disk_auto_clean_percent": 90.0,
  "ram_standby_purge_threshold_mb": 1024
}
```

Runtime service logs are written directly to:
```
C:\ssm\ssm.log
```
</details>

---

## ❓ Troubleshooting & FAQ

<details>
<summary><b>1. Why does ssm require Administrator privileges?</b></summary>

`ssm` interacts directly with Win32 NT kernel APIs (such as `NtSetTimerResolution`, `PowerWriteACValueIndex`, PCI Registry for MSI-X, and Large Page memory tokens). Windows requires elevation to modify these low-level parameters.
</details>

<details>
<summary><b>2. How do I verify that ssm is running in the background?</b></summary>

Run:
```powershell
sc query SmartSystemManager
```
Or check live telemetry with:
```powershell
ssm stats
```
</details>

<details>
<summary><b>3. Is ssm safe for modern Antivirus software?</b></summary>

Yes. `ssm` is 100% open-source native Rust code with zero telemetry and no network activity other than checking official GitHub releases. Because it optimizes low-level NT APIs and Registry settings, some heuristic scanners may flag system-tuning tools. You can safely whitelist `C:\ssm\ssm.exe`.
</details>

---

## 🗑️ Clean Uninstallation

To completely remove `ssm` and revert all system tweaks back to default Windows values:

```powershell
ssm uninstall
```

This will automatically:
1. Revert registry settings and power indices to Windows defaults
2. Stop and delete the `SmartSystemManager` Windows service
3. Remove `C:\ssm` from your system `PATH`
4. Clean up `C:\ssm\` files

---

## 📄 License

Distributed under the **MIT License**. See [`LICENSE`](LICENSE) for details.

<div align="center">
  <sub>Built with ❤️ using 100% Native Rust for Windows.</sub>
</div>
