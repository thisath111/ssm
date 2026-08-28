# Smart System Manager (`ssm`)

**High-Performance Windows System Optimizer & Low-Latency Daemon**  
Written in 100% Native Rust · Zero UI · Zero Bloat · Zero Telemetry · Native Windows Service

[![Build Status](https://github.com/thisath111/ssm/actions/workflows/release.yml/badge.svg)](https://github.com/thisath111/ssm/actions)
[![Tests](https://github.com/thisath111/ssm/actions/workflows/audit.yml/badge.svg)](https://github.com/thisath111/ssm/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Platform: Windows x64](https://img.shields.io/badge/Platform-Windows_10_%7C_11_(x64)-0078D6)](https://microsoft.com/windows)
[![Release](https://img.shields.io/github/v/release/thisath111/ssm)](https://github.com/thisath111/ssm/releases/latest)

---

## Table of Contents
1. [Overview](#overview)
2. [Quick Installation (Automated 1-Liner)](#quick-installation-automated-1-liner)
3. [Manual Installation (Complete Step-by-Step Guide)](#manual-installation-complete-step-by-step-guide)
4. [Building from Source (Rust Developer Guide)](#building-from-source-rust-developer-guide)
5. [CLI Commands Reference](#cli-commands-reference)
6. [Automatic & Manual Updates](#automatic--manual-updates)
7. [Core Optimization Features](#core-optimization-features)
8. [Configuration & File Logging](#configuration--file-logging)
9. [Troubleshooting & FAQ](#troubleshooting--faq)
10. [Uninstallation](#uninstallation)
11. [License](#license)

---

## Overview

`ssm` is a lightweight, autonomous background optimization daemon that applies deep hardware- and kernel-level tuning that Windows does not do by default.

A **Pure-Rust AI engine** continuously monitors system activity and classifies active workloads (Gaming, Video Editing, Software Development, or Idle) to dynamically adjust CPU core affinities, GPU queues, timer resolutions, and memory pressure in real time.

### Resource Usage:
- **CPU Usage:** Less than `0.01%`
- **Memory Footprint:** Approximately `6 MB RAM`
- **Runtime:** Native binary (no .NET, no Electron, no WebView, no external dependencies)
- **Execution Mode:** Runs as a native background Windows Service with boot autostart

---

## Quick Installation (Automated 1-Liner)

The fastest and easiest way to install `ssm` is via PowerShell.

1. Open **PowerShell as Administrator** (Right-click Start button → select *Terminal (Admin)* or *Windows PowerShell (Admin)*).
2. Paste and run the following command:

```powershell
irm https://raw.githubusercontent.com/thisath111/ssm/main/install.ps1 | iex
```

### What this script does automatically:
- Creates the folder `C:\ssm\`
- Downloads the latest `ssm.exe` binary from GitHub Releases
- Adds `C:\ssm` to the system `PATH` environment variable permanently
- Registers `SmartSystemManager` as a Windows Native Service configured for boot autostart
- Starts the optimization service immediately

---

## Manual Installation (Complete Step-by-Step Guide)

If you prefer to install `ssm` manually without using an automated script, follow these detailed steps:

### Step 1: Download `ssm.exe`
1. Open your web browser and navigate to the [GitHub Releases Page](https://github.com/thisath111/ssm/releases/latest).
2. Under the **Assets** section, click on **`ssm.exe`** to download the pre-compiled binary.

### Step 2: Create a Dedicated Folder and Place `ssm.exe`
1. Open File Explorer and navigate to your `C:\` drive.
2. Create a new folder named `ssm` (so the full path is `C:\ssm\`).
3. Move the downloaded `ssm.exe` from your `Downloads` folder into `C:\ssm\`.

*Alternatively, you can do this using PowerShell as Administrator:*
```powershell
New-Item -ItemType Directory -Force -Path "C:\ssm"
Move-Item -Path "$HOME\Downloads\ssm.exe" -Destination "C:\ssm\ssm.exe"
```

### Step 3: Add `C:\ssm` to Windows System PATH
Adding `C:\ssm` to your `PATH` allows you to run `ssm` commands from any terminal or PowerShell window without typing the full file path.

#### Method A: Using PowerShell (Recommended)
Open **PowerShell as Administrator** and execute:
```powershell
[Environment]::SetEnvironmentVariable(
    "Path",
    [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::Machine) + ";C:\ssm",
    [EnvironmentVariableTarget]::Machine
)
```

#### Method B: Using Windows Graphical Interface (GUI)
1. Press `Win + R` on your keyboard, type `sysdm.cpl`, and press **Enter**.
2. In the System Properties window, click on the **Advanced** tab.
3. Click the **Environment Variables...** button near the bottom.
4. In the **System variables** section (bottom box), locate and select the `Path` variable, then click **Edit...**.
5. Click **New** on the right side, and type: `C:\ssm`
6. Click **OK** on all three open windows to save the changes.
7. Restart any open PowerShell or Command Prompt windows for the new PATH to take effect.

### Step 4: Register and Start the Windows Service
To have `ssm` continuously optimize your system in the background and auto-start on every boot:

Open **PowerShell as Administrator** and run:
```powershell
ssm service install
```

*This configures the Windows Service Control Manager (SCM) to run `ssm.exe daemon` under the LocalSystem account with automatic recovery on failure.*

### Step 5: Verify the Installation
Run the following command to check if `ssm` is working and view live hardware telemetry:
```powershell
ssm stats
```

You should see a live display of CPU topology, memory pressure, active process intent, and kernel timer resolution.

---

## Building from Source (Rust Developer Guide)

If you want to compile `ssm` from source code yourself:

### Prerequisites
1. **Rust & Cargo:** Install from [https://rustup.rs/](https://rustup.rs/) (Rust 1.75 or newer).
2. **Visual Studio C++ Build Tools:** Required for MSVC linking on Windows (Download from [visualstudio.microsoft.com](https://visualstudio.microsoft.com/visual-cpp-build-tools/) and select *Desktop development with C++*).
3. **Git:** Download and install from [https://git-scm.com/](https://git-scm.com/).

### Compilation & Installation Steps

1. **Clone the repository:**
   ```powershell
   git clone https://github.com/thisath111/ssm.git
   cd ssm
   ```

2. **Run all automated tests:**
   ```powershell
   cargo test
   ```
   *Ensures all 13 unit tests for AI intent classification, memory forecasting, and kernel sensors pass.*

3. **Compile the optimized release binary:**
   ```powershell
   cargo build --release
   ```
   The compiled executable will be generated at:
   `target\release\ssm.exe`

4. **Deploy the compiled binary to `C:\ssm`:**
   ```powershell
   New-Item -ItemType Directory -Force -Path "C:\ssm"
   Copy-Item ".\target\release\ssm.exe" "C:\ssm\ssm.exe" -Force
   ssm service install
   ```

---

## CLI Commands Reference

All commands must be run in **PowerShell or Command Prompt as Administrator** to allow access to low-level NT kernel APIs and system registries.

```powershell
ssm <COMMAND> [OPTIONS]
```

### Complete Command Matrix:

| Command | Arguments / Flags | Description |
|---|---|---|
| `ssm stats` | — | Displays live system hardware telemetry, CPU topology (P/E cores), AI active workload intent, RAM pressure, and kernel timer resolution. |
| `ssm boost` | — | Applies an immediate one-click full performance boost (Unparks CPU cores, locks 0.5ms Timer, enables Large Pages, enables DirectFlip DWM). |
| `ssm clean` | — | Instantly purges Windows Standby RAM cache and cleans temporary junk files. |
| `ssm tune` | — | Applies permanent low-latency system registry tweaks (MSI-X mode, TCP NoDelay, Fast Shutdown, USB latency fix). |
| `ssm update` | — | Checks GitHub for new releases and performs an atomic in-place update of `ssm.exe`. |
| `ssm update` | `--check` | Checks if a newer version is available on GitHub without downloading or installing it. |
| `ssm update` | `--enable` | Enables automatic weekly background update checks (enabled by default). |
| `ssm update` | `--disable` | Disables background update checks. |
| `ssm daemon` | — | Runs the background optimization engine directly in the console (useful for real-time monitoring or debugging). |
| `ssm service install` | — | Registers `ssm` as an autostarting Windows Native Service and starts it. |
| `ssm service uninstall` | — | Stops and removes the `SmartSystemManager` Windows Service. |
| `ssm uninstall` | — | Completely uninstalls `ssm`: restores registry settings, removes service, and deletes `C:\ssm`. |

---

## Automatic & Manual Updates

`ssm` features a built-in atomic self-updating mechanism designed to work safely on Windows.

### How Background Auto-Update Works:
- **Default State:** Enabled automatically when you install `ssm`.
- **Frequency:** Every 7 days, the background daemon queries the GitHub Releases API.
- **Atomic Binary Replacement:** Windows prevents overwriting running executables. `ssm` uses an atomic `self_replace` pattern that renames the active executable to `.old`, writes the new binary in place, and restarts the service.

### Update Commands:
- **Check and install latest update:**
  ```powershell
  ssm update
  ```
- **Check for updates without installing:**
  ```powershell
  ssm update --check
  ```
- **Disable auto-updates:**
  ```powershell
  ssm update --disable
  ```
- **Re-enable auto-updates:**
  ```powershell
  ssm update --enable
  ```

---

## Core Optimization Features

### 1. ⚡ Hardware-Adaptive Kernel Tuning
- **0.5ms Win32 Timer Resolution:** Locks system timer via `NtSetTimerResolution` to 0.5ms / 0.75ms / 1.0ms based on hardware tier for ultra-low input latency.
- **Interrupt Affinity Routing:** Dynamically routes GPU and Network PCI interrupts directly to Performance Cores (`p_core_mask`), eliminating Core 0 DPC latency bottlenecks.
- **CPU C-State & Idle Suppression:** Suppresses deep CPU idle C-states during high-performance workloads via native power index calls to eliminate micro-stuttering.
- **MMCSS Real-Time Scheduling:** Registers the engine thread with the Windows Multimedia Class Scheduler Service (`AvSetMmThreadCharacteristicsW`) for hard real-time execution priority.
- **P-Core / E-Core Optimization:** Automatically detects CPU topology and pins background tasks to Efficiency Cores, reserving Performance Cores for foreground tasks.

### 2. 🧠 Zero-Hardcode AI Engine & Behavioral Heuristics
- **Behavioral Intent Classifier:** Identifies Gaming, Video/Audio Production, Development, or Typing Tool workloads purely from behavioral telemetry (memory velocity, thread activity, window ownership) — no hardcoded process name lists.
- **Typing Tool Immunity:** Dynamic Win32 window ownership scanning detects all IME and typing tools (Helakuru, Keyman, Wijesekara, etc.) and protects them from throttling.
- **Predictive Standby Memory Forecaster:** Computes RAM velocity momentum ($\frac{dM}{dt}$) to forecast memory pressure 5 seconds ahead and pre-emptively purges standby cache before stutter occurs.

### 3. 🚀 Deep Windows Subsystem Optimizations
- **Global Timer Resolution (Win 11 Fix):** Enforces global 0.5ms timer resolution across all processes via kernel Session Manager registry.
- **Win32 Priority Separation (`0x26`):** Enhances foreground thread responsiveness by prioritizing interactive foreground quantum.
- **DirectFlip Zero-Lag DWM:** Bypasses Desktop Window Manager composition delays and routes frame queues directly to GPU scan-out.
- **2MB Large Pages:** Unlocks `SeLockMemoryPrivilege` for Large Page memory allocations, reducing CPU TLB cache misses.
- **GPU & Network MSI-X Mode:** Converts adapters to Message Signaled Interrupts via PCI Registry to eliminate DPC latency spikes.
- **TCP NoDelay & QoS DSCP:** Disables Nagle algorithm and prioritizes latency-critical packets.

### 4. 🛡️ Stability & Anti-Hang Shield
- **Explorer Auto-Rescue:** Continuously monitors `explorer.exe` message loops via `IsHungAppWindow` and auto-rescues frozen taskbars within seconds.
- **Zero-Downtime Supervised Engine:** Wraps all daemon routines in panic-isolated supervisor loops (`catch_unwind`) with automatic recovery.
- **Background Disk Hog Throttling:** Automatically deprioritizes background disk hogs to `IDLE_PRIORITY_CLASS` and lowest I/O priority.
- **Fast Shutdown:** Reduces service shutdown timeouts to 2 seconds and enables `AutoEndTasks`.

---

## Configuration & File Logging

### Configuration File (`config.json`)
The configuration file is stored in `C:\ssm\config.json`. The daemon hot-reloads this file every 10 seconds without needing a service restart.

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

### Runtime Logs (`ssm.log`)
When running as a background service, logs are continuously written to:
`C:\ssm\ssm.log`

---

## Troubleshooting & FAQ

### Q1: Why do I get "Access is denied" when running commands?
`ssm` modifies low-level NT kernel parameters (Timer Resolution, PCI Registry, Power GUIDs). You must run PowerShell or Command Prompt **as Administrator**.

### Q2: How can I verify that the service is running?
Run:
```powershell
sc query SmartSystemManager
```
Or check live engine stats with:
```powershell
ssm stats
```

### Q3: Is `ssm` safe with modern Antivirus software?
Yes. `ssm` is 100% open-source native Rust code with zero telemetry and no malicious activity. Because it optimizes low-level NT APIs and Registry settings, some heuristic scanners may flag system-tuning utilities. You can safely add `C:\ssm\ssm.exe` to your antivirus exclusion list.

---

## Uninstallation

To completely uninstall `ssm` and revert all system tweaks to Windows default values:

Open **PowerShell as Administrator** and run:
```powershell
ssm uninstall
```

This will automatically:
1. Revert registry settings and power indices to Windows defaults
2. Stop and delete the `SmartSystemManager` Windows service
3. Remove `C:\ssm` from your system `PATH`
4. Clean up `C:\ssm\` files

---

## License

Distributed under the **MIT License**. See [`LICENSE`](LICENSE) for details.
