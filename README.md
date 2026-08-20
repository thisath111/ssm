# 🚀 Smart System Manager (`ssm`) v1.0.0

[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%2010%20%7C%2011-blue.svg)](https://microsoft.com)
[![Build](https://img.shields.io/github/actions/workflow/status/thisath111/ssm/release.yml?label=build)](https://github.com/thisath111/ssm/releases/latest)

A production-grade, ultra-low latency Windows performance optimizer and background daemon written entirely in **100% Native Rust**. `ssm` combines sub-millisecond Win32 timer resolution, AI-assisted process scheduling, GPU MSI-X interrupt tuning, zero-copy RAM purging, and an Anti-Hang Stability Shield — all running silently in the background.

---

## ⚡ Quick 1-Line Installation (Windows PowerShell)

Run this single command in an **Administrator PowerShell** to automatically download `ssm` into `C:\ssm\ssm.exe`, register it in your Windows `PATH`, install it as a **Windows Service**, and enable **boot autostart** — all in one shot:

```powershell
irm https://raw.githubusercontent.com/thisath111/ssm/main/install.ps1 | iex
```

> **Note:** The installer automatically handles everything. You do **not** need to run `ssm service install` manually.

---

### 📦 Manual Installation (Without Installer Script)

If you prefer to set up `ssm` manually:

1. Download **`ssm.exe`** from [GitHub Releases](https://github.com/thisath111/ssm/releases/latest).
2. Create the folder `C:\ssm\` and move `ssm.exe` inside it (`C:\ssm\ssm.exe`).
3. Add `C:\ssm` to your Windows User `PATH` environment variable:
   - **PowerShell (run once):**
     ```powershell
     [Environment]::SetEnvironmentVariable("Path", "$([Environment]::GetEnvironmentVariable('Path', 'User'));C:\ssm", "User")
     ```
4. **Restart** your PowerShell / Command Prompt so the new PATH takes effect.
5. **Run the service installation command:**
   ```powershell
   ssm service install
   ```

After step 5, `ssm` will start automatically every time Windows boots.

---

## ✨ Features

### ⚡ Ultra-Low Latency Engine
- **Sub-Millisecond Timer Resolution (0.5ms):** Direct NT API timer resolution (`NtSetTimerResolution(5000)`), mouse acceleration bypass, and Win32 priority quantum tuning (`Win32PrioritySeparation = 0x26`).
- **GPU MSI-X Interrupt Optimization:** Forces GPU and Network adapters to use Message Signaled Interrupts (MSI) instead of legacy line-based IRQs — completely eliminates DPC Latency spikes and micro-stutters in games.

### 🤖 AI-Assisted Smart Process Scheduler
- **App Launch Accelerator:** Detects newly launched processes in real-time and instantly boosts them to `HIGH_PRIORITY_CLASS` for 3 seconds, making every application snap open instantly.
- **Background Burst Throttling:** Automatically detects and throttles known CPU/disk-hogging Windows background services (`TiWorker.exe`, `compattelrunner.exe`, `MoUsoCoreWorker.exe`, etc.) to `IDLE_PRIORITY_CLASS` — eliminating the "internet-on" freeze effect.
- **Dynamic Foreground Focus:** Pins the active window's process to P-Cores and gives it maximum I/O priority at all times.

### 🛡️ Anti-Hang Stability Shield
- **Explorer Auto-Rescue:** Monitors `explorer.exe` via `IsHungAppWindow`. If the Windows Taskbar freezes (e.g. due to a damaged USB drive), `ssm` automatically kills and restarts it within seconds — your desktop is never stuck.
- **Bad USB / I/O Deadlock Prevention:** Detects disk I/O deadlocks from damaged removable media and deprioritizes their I/O load before it can freeze the entire OS.
- **Zero-Crash OS Guard:** System handle leak auditing (>10,000 handles) and core immunity shield protecting critical Windows processes (`csrss`, `lsass`, `dwm`, `winlogon`).

### 🏎️ CPU & Memory Optimization
- **P-Core / E-Core Topology Affinity:** Detects P-Cores vs E-Cores via `GetLogicalProcessorInformationEx` and pins background hogs to E-Cores, freeing P-Cores for your active workload.
- **Mathematical Predictive Core:** Integrated PID Control Loop and 1D Kalman Filter for workload estimation and proactive resource allocation.
- **RAM & Storage Acceleration:** Zero-copy Standby Memory purging, rate-limited WorkingSet compression, NVMe queue depth tuning, and NTFS MFT cache maximization.

### 🔧 System Tweaks
- **Fast Shutdown:** Reduces `WaitToKillServiceTimeout` and `WaitToKillAppTimeout` to 2 seconds and enables `AutoEndTasks` — so your PC shuts down in seconds, not minutes.
- **Network Optimization:** TCP NoDelay / Nagle Algorithm bypass, QoS DSCP CS1 packet prioritization, and Network Throttling Index removal.
- **Telemetry & Search Cleanup:** Disables Windows Telemetry, Cortana, and Search Indexer spikes that cause random CPU usage bursts.

---

## 💻 CLI Usage

```powershell
# Display live system hardware status, timer resolution & Kalman load
ssm stats

# One-click instant ultra-low latency system & hardware boost
ssm boost

# Purge Standby memory list & clean temp storage files
ssm clean

# Apply low-latency system registry tweaks (MSI Mode, Fast Shutdown, etc.)
ssm tune

# Run as a background optimization daemon
ssm daemon

# Register as Windows Service + enable boot autostart
ssm service install
ssm service uninstall

# Complete uninstallation (reverts tweaks, removes service/autostart/PATH, deletes C:\ssm)
ssm uninstall

# Display help and available commands
ssm --help
```

---

## 🛠️ Building from Source

### Prerequisites
- [Rust 2021 Edition](https://www.rust-lang.org/tools/install) (1.75+)
- Windows 10 / 11 (64-bit)

### Build Instructions

```powershell
# Clone the repository
git clone https://github.com/thisath111/ssm.git
cd ssm

# Build optimized release binary
cargo build --release
```

The release binary will be available at `./target/release/ssm.exe`.

> **Note:** The official release binary is built on GitHub Actions with `target-cpu=native` optimizations for maximum performance.

---

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.
