# 🚀 Smart System Manager (`ssm`) v1.0.0

[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%2010%20%7C%2011-blue.svg)](https://microsoft.com)

A production-grade, ultra-low latency Windows performance optimizer and background daemon written entirely in **100% Native Rust**. `ssm` combines sub-millisecond Win32 timer resolution, P-Core/E-Core CPU affinity pinning, zero-copy Standby memory purging, NVMe storage acceleration, and a Zero-Crash OS Guard.

---

## ⚡ Quick 1-Line Installation (Windows PowerShell)

Run this single command in an **Administrator PowerShell** to automatically download `ssm` into `C:\ssm\ssm.exe`, register it in your Windows `PATH`, install it as a **Windows Service**, and enable **boot autostart** — all in one shot:

```powershell
irm https://raw.githubusercontent.com/thisath111/ssm/main/install.ps1 | iex
```

> **Note:** `ssm service install` is automatically called at the end of the installer. You do **not** need to run it manually.

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
   - **Windows GUI:** Press `Win + R` → type `sysdm.cpl` → **Advanced** → **Environment Variables** → Select **Path** under User variables → **Edit** → **New** → type `C:\ssm` → **OK**.
4. **Restart** your PowerShell / Command Prompt so the new PATH takes effect.
5. **Run the service installation command** (required — registers Windows Service and enables boot autostart):
   ```powershell
   ssm service install
   ```

After step 5, `ssm` will start automatically every time Windows boots.

---

## ✨ Features

- **⚡ Sub-Millisecond Input Latency (0.5ms):** Direct NT API timer resolution (`NtSetTimerResolution(5000)`), mouse acceleration bypass, and Win32 priority quantum tuning (`Win32PrioritySeparation = 0x26`).
- **🧠 Mathematical Predictive Core:** Integrated PID Control Loop, 1D Kalman Filtering for workload estimation, and Page Fault Rate Calculus (\( \frac{dPF}{dt} \)).
- **🛡️ Zero-Crash OS Guard:** System handle leak auditing (>10,000 handles) and core immunity shield protecting critical Windows processes (`csrss`, `lsass`, `dwm`, `winlogon`).
- **🏎️ CPU & Core Topology Affinity:** Detects P-Cores vs E-Cores / AMD CCX structures via `GetLogicalProcessorInformationEx` and pins background hogs away from performance cores.
- **🎮 GPU & DirectX Optimization:** Hardware-Accelerated GPU Scheduling (HAGS) tuning, DirectX/DWM GPU priority escalation, and game task profile boosting.
- **🧹 RAM & Storage Acceleration:** Zero-copy Standby Memory purging, rate-limited WorkingSet compression, NVMe queue depth tuning, and NTFS MFT cache maximization.

---

## 💻 CLI Usage

```powershell
# Display help and available commands
ssm --help

# Display live system hardware status, timer resolution & Kalman load
ssm stats

# One-click instant ultra-low latency system & hardware boost
ssm boost

# Purge Standby memory list & clean temp storage files
ssm clean

# Apply low-latency system registry tweaks
ssm tune

# Run as a background optimization daemon
ssm daemon

# Register as Windows Service + enable boot autostart
ssm service install
ssm service uninstall

# Complete uninstallation (reverts tweaks, removes service/autostart/PATH, deletes C:\ssm)
ssm uninstall
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

---

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.
