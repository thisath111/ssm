# Smart System Manager (`ssm`)

> High-performance, AI-driven Windows system optimizer and low-latency background daemon written in 100% Native Rust.  
> Runs silently as an autonomous Windows Native Service — zero UI, zero telemetry, zero bloat, maximum performance.

[![Build](https://github.com/thisath111/ssm/actions/workflows/release.yml/badge.svg)](https://github.com/thisath111/ssm/actions/workflows/release.yml)
[![Tests](https://github.com/thisath111/ssm/actions/workflows/audit.yml/badge.svg)](https://github.com/thisath111/ssm/actions/workflows/audit.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows_10_%7C_11_(x64)-0078D6)](https://microsoft.com/windows)
[![Release](https://img.shields.io/github/v/release/thisath111/ssm)](https://github.com/thisath111/ssm/releases/latest)

---

## Table of Contents
- [What is ssm?](#what-is-ssm)
- [Quick Installation (Automated 1-Liner)](#quick-installation-automated-1-liner)
- [Manual Installation (Step-by-Step Guide)](#manual-installation-step-by-step-guide)
- [Building from Source (Rust Developer Guide)](#building-from-source-rust-developer-guide)
- [CLI Reference & Usage](#cli-reference--usage)
- [Automatic & Manual Updates](#automatic--manual-updates)
- [Core Features & Architecture](#core-features--architecture)
- [Configuration & Logging](#configuration--logging)
- [Troubleshooting & FAQ](#troubleshooting--faq)
- [Uninstallation](#uninstallation)
- [License](#license)

---

## What is ssm?

`ssm` is a background Windows optimization daemon that applies deep kernel-level tuning that Windows never does out of the box. A **Pure-Rust AI engine** classifies what you are doing in real time — Gaming, Coding, Video Editing, or Idle — then dynamically adjusts CPU priority, GPU scheduling, RAM pressure, and network quality. Automatically. No configuration needed.

Unlike bloatware "optimizer" apps, `ssm` has no GUI, no ads, no telemetry, and no dependencies. Open-source, written entirely in Rust, consuming less than `0.01% CPU` and `~6 MB RAM`.

---

## Quick Installation (Automated 1-Liner)

The fastest way to install `ssm` is using PowerShell.

Open **PowerShell as Administrator** (Right-click Start menu → *Terminal (Admin)* or *Windows PowerShell (Admin)*) and run:

```powershell
irm https://raw.githubusercontent.com/thisath111/ssm/main/install.ps1 | iex
```

### What this script does automatically:
1. Creates directory `C:\ssm\`
2. Downloads the latest pre-compiled `ssm.exe` binary from GitHub Releases
3. Adds `C:\ssm` permanently to your Windows System `PATH`
4. Registers `SmartSystemManager` as a Windows Native Service (SCM) configured to auto-start on every system boot
5. Starts the optimization daemon immediately in the background

---

## Manual Installation (Step-by-Step Guide)

If you prefer to install `ssm` manually without running an automated script, follow these steps:

### Step 1: Download the Binary
1. Go to the [Releases Page](https://github.com/thisath111/ssm/releases/latest).
2. Download the **`ssm.exe`** file from the Assets section.

### Step 2: Place `ssm.exe` in a Permanent Folder
1. Create a dedicated folder on your system drive, e.g., `C:\ssm\`.
2. Move the downloaded `ssm.exe` into `C:\ssm\`.

### Step 3: Add `C:\ssm` to System PATH

#### Option A: Using PowerShell (Administrator)
```powershell
[Environment]::SetEnvironmentVariable(
    "Path",
    [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::Machine) + ";C:\ssm",
    [EnvironmentVariableTarget]::Machine
)
```

#### Option B: Using Windows GUI
1. Press `Win + R`, type `sysdm.cpl`, and press Enter.
2. Go to the **Advanced** tab and click **Environment Variables...**.
3. Under **System variables**, select `Path` and click **Edit...**.
4. Click **New**, enter `C:\ssm`, and click **OK** on all open windows.

### Step 4: Register as a Windows Service (Boot Autostart)
Open **PowerShell as Administrator** and run:
```powershell
ssm service install
```
This registers `SmartSystemManager` in the Windows Service Control Manager (SCM) and starts it immediately.

### Step 5: Verify Installation
Verify that `ssm` is running and accessible:
```powershell
ssm stats
```

---

## Building from Source (Rust Developer Guide)

You can easily compile `ssm` from source code using the Rust toolchain.

### Prerequisites
1. **Rust & Cargo**: Install from [https://rustup.rs/](https://rustup.rs/) (Rust 1.75+ recommended).
2. **Visual Studio C++ Build Tools**: Required for MSVC linker on Windows (Installed via Visual Studio Installer → *Desktop development with C++*).
3. **Git**: Install from [https://git-scm.com/](https://git-scm.com/).

### Compilation Steps

1. **Clone the repository:**
   ```powershell
   git clone https://github.com/thisath111/ssm.git
   cd ssm
   ```

2. **Run the automated unit tests:**
   ```powershell
   cargo test
   ```
   *All 13 unit tests will execute and validate the AI intent classifier, memory forecaster, and kernel sensors.*

3. **Build the optimized release binary:**
   ```powershell
   cargo build --release
   ```
   The compiled standalone binary will be located at:
   ```
   target\release\ssm.exe
   ```

4. **Install the compiled binary:**
   Copy the binary to `C:\ssm\` and register the service:
   ```powershell
   New-Item -ItemType Directory -Force -Path "C:\ssm"
   Copy-Item ".\target\release\ssm.exe" "C:\ssm\ssm.exe"
   ssm service install
   ```

---

## CLI Reference & Usage

All commands require **Administrator privileges** to access Win32 NT kernel APIs and system registries.

```powershell
ssm [COMMAND] [OPTIONS]
```

### Available Commands

| Command | Description |
|---|---|
| `ssm stats` | Display live hardware telemetry, AI active app intent, CPU/RAM stats, and kernel timer resolution. |
| `ssm boost` | Run one-click deep system optimization (Unparks CPU cores, locks 0.5ms Timer, enables Large Pages, configures DWM DirectFlip). |
| `ssm clean` | Instantly purges Windows Standby RAM cache and clears system temporary file junk. |
| `ssm tune` | Applies permanent low-latency registry tweaks (MSI-X mode, TCP NoDelay, Fast Shutdown, USB latency fix). |
| `ssm update` | Checks GitHub for new releases and performs an atomic in-place update. |
| `ssm update --check` | Checks if a newer version is available without downloading or installing. |
| `ssm update --enable` | Enables automatic weekly background update checks (enabled by default). |
| `ssm update --disable` | Disables background update checks. |
| `ssm daemon` | Runs the optimization daemon in foreground interactive console mode (useful for debugging). |
| `ssm service install` | Installs and starts `ssm` as a Windows Native Service with boot autostart. |
| `ssm service uninstall` | Stops and removes the Windows Service. |
| `ssm uninstall` | Completely uninstalls `ssm`: reverts registry tweaks, deletes service, and cleans `PATH`. |

---

## Automatic & Manual Updates

`ssm` includes a production-grade self-updating engine designed specifically for Windows.

### How Background Auto-Update Works
- **Default State**: Enabled automatically upon installation.
- **Schedule**: Every 7 days, the background daemon checks the GitHub API for newer releases.
- **In-Place Atomic Replacement**: Because Windows locks running executables, `ssm` renames the active binary to `.old`, writes the new executable, and restarts the Windows service without downtime.

### Manual Update Commands
- Check for updates manually:
  ```powershell
  ssm update
  ```
- Check version without installing:
  ```powershell
  ssm update --check
  ```
- Turn off auto-updates:
  ```powershell
  ssm update --disable
  ```
- Turn on auto-updates:
  ```powershell
  ssm update --enable
  ```

---

## Core Features & Architecture

### ⚡ 1. Hardware-Adaptive Kernel Tuning
- **Adaptive Timer Resolution**: Locks system timer via `NtSetTimerResolution` at 0.5ms / 0.75ms / 1.0ms based on hardware tier.
- **Interrupt Affinity Optimization**: Dynamically routes GPU and Network PCI interrupts directly to Performance Cores (`p_core_mask`), preventing Core 0 DPC latency bottlenecks.
- **CPU C-State & Idle Suppression**: Suppresses deep CPU idle states during high-performance workloads to eliminate frame stuttering.
- **MMCSS Thread Scheduling Boost**: Registers the engine thread with the Windows Multimedia Class Scheduler Service (`AvSetMmThreadCharacteristicsW`) for hard real-time execution priority.
- **P-Core / E-Core Topology Optimization**: Detects CPU architecture and pins background tasks to Efficiency Cores, reserving Performance Cores for interactive foreground workloads.

### 🧠 2. Zero-Hardcode AI Engine
- **Behavioral Intent Classifier**: Identifies Gaming, Video/Audio Production, Development, or Typing Tool workloads purely from behavioral telemetry (memory velocity, thread activity, window ownership) — zero hardcoded process name lists.
- **Typing Tool Immunity**: Dynamic Win32 window ownership scanning detects all IME and typing tools (Helakuru, Keyman, Wijesekara, etc.) and protects them from throttling or suspension.
- **Predictive Standby Memory Forecaster**: Calculates RAM momentum ($\frac{dM}{dt}$) to forecast memory pressure 5 seconds ahead and purges standby cache *before* stutter occurs.

### 🚀 3. Deep Windows Subsystem Optimizations
- **Global Timer Resolution (Win 11 Fix)**: Enforces global 0.5ms timer resolution across all processes via kernel Session Manager registry.
- **Win32 Priority Separation (`0x26`)**: Enhances foreground thread responsiveness by prioritizing interactive foreground quantum.
- **DirectFlip Zero-Lag DWM**: Bypasses Desktop Window Manager composition delays and routes frame queues directly to GPU scan-out.
- **2MB Large Pages**: Unlocks `SeLockMemoryPrivilege` to reduce CPU TLB cache misses for compilers and game engines.
- **GPU & Network MSI-X Mode**: Converts adapters to Message Signaled Interrupts via PCI Registry to eliminate DPC latency spikes.
- **TCP NoDelay & QoS DSCP**: Disables Nagle algorithm and prioritizes latency-critical packets.

### 🛡️ 4. Stability & Anti-Hang Shield
- **Explorer Auto-Rescue**: Continuously monitors `explorer.exe` message loops via `IsHungAppWindow` and auto-rescues frozen taskbars within seconds.
- **Supervised Microkernel**: Wraps all daemon routines in panic-isolated supervisor loops (`catch_unwind`) to ensure zero downtime.
- **Background Disk Hog Throttling**: Automatically deprioritizes background disk hogs to `IDLE_PRIORITY_CLASS` and lowest I/O priority.
- **Fast Shutdown**: Reduces service shutdown timeouts to 2 seconds and enables `AutoEndTasks`.

---

## Configuration & Logging

### Configuration File (`config.json`)
The configuration file is automatically created in the executable directory (e.g. `C:\ssm\config.json`). The daemon hot-reloads this file every 10 seconds without needing a restart.

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

### Log Files (`ssm.log`)
In Service mode, runtime logs are continuously written to:
```
C:\ssm\ssm.log
```

---

## Troubleshooting & FAQ

### 1. "Access is denied" / Privileges error
`ssm` interacts directly with Win32 NT kernel APIs (e.g., `NtSetTimerResolution`, `PowerWriteACValueIndex`, PCI Registry). **Always run PowerShell or Command Prompt as Administrator.**

### 2. How do I check if the background service is running?
Run:
```powershell
sc query SmartSystemManager
```
Or view live status via:
```powershell
ssm stats
```

### 3. Will this conflict with Antivirus software?
`ssm` is 100% open-source native Rust code with no malware or telemetry. However, because it modifies system registry keys (MSI-X interrupts, Fast Shutdown) and uses low-level NT APIs, some heuristic antivirus scanners may flag it. You can safely add `C:\ssm\ssm.exe` to your antivirus exclusion list.

---

## Uninstallation

To completely remove `ssm` and revert all system tweaks to Windows defaults:

Open **PowerShell as Administrator** and run:
```powershell
ssm uninstall
```
This will:
1. Revert registry tweaks and power settings back to Windows defaults
2. Stop and delete the `SmartSystemManager` Windows service
3. Remove `C:\ssm` from your system `PATH`
4. Clean up `C:\ssm\` directory

---

## License

Distributed under the **MIT License**. See [`LICENSE`](LICENSE) for details.
