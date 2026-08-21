# Smart System Manager (`ssm`)

> High-Performance, AI-Driven Windows System Optimizer written in 100% Native Rust.

[![Build](https://github.com/thisath111/ssm/actions/workflows/release.yml/badge.svg)](https://github.com/thisath111/ssm/actions)
[![Tests](https://github.com/thisath111/ssm/actions/workflows/audit.yml/badge.svg)](https://github.com/thisath111/ssm/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows_10_%7C_11-blue)](https://www.microsoft.com/windows)

`ssm` is an open-source background daemon that combines sub-millisecond Win32 kernel tuning, a Pure-Rust AI process scheduler, and a stability shield — running silently with near-zero overhead.

---

## Installation

```powershell
irm https://raw.githubusercontent.com/thisath111/ssm/main/install.ps1 | iex
```

Requires **Administrator PowerShell**. Installs to `C:\ssm`, registers a Windows Service, and enables boot autostart.

---

## Features

| Feature | Description |
|---|---|
| **0.5ms Timer Resolution** | Locks Win32 timer to sub-millisecond via `NtSetTimerResolution(5000)` |
| **AI Process Scheduler** | Classifies foreground intent (Gaming / Dev / Creative) and adapts CPU/GPU priority |
| **Predictive RAM Purge** | Forecasts memory pressure spikes 5s ahead and pre-emptively flushes standby cache |
| **DWM DirectFlip** | Eliminates Desktop Window Manager animation delays for zero input lag |
| **Large Page Memory** | Grants `SeLockMemoryPrivilege` to enable 2MB/4MB kernel HugePages |
| **MSI-X Interrupts** | Converts GPU & Network adapters to Message Signaled Interrupts via PCI registry |
| **Anti-Hang Shield** | Auto-rescues frozen `explorer.exe` from USB I/O deadlocks within seconds |
| **Fast Shutdown** | Kills stuck services in 2s via `AutoEndTasks` and reduced `WaitToKillTimeout` |
| **P/E Core Affinity** | Pins background processes to E-Cores, freeing P-Cores for foreground work |
| **Network Burst Throttle** | Permanently locks Windows telemetry/update services to `IDLE_PRIORITY_CLASS` |

---

## CLI Usage

```powershell
ssm stats            # Live hardware status, AI intent, timer resolution
ssm boost            # Instant one-click performance boost
ssm clean            # Purge standby RAM & temp files
ssm tune             # Apply registry tweaks (MSI mode, fast shutdown, TCP NoDelay)
ssm service install  # Register as Windows background service
ssm service uninstall
ssm uninstall        # Remove ssm completely
```

---

## Building from Source

**Requirements:** [Rust 1.75+](https://rustup.rs/) · Windows 10/11 (64-bit)

```powershell
git clone https://github.com/thisath111/ssm.git
cd ssm
cargo build --release
cargo test
```

---

## License

MIT — see [LICENSE](LICENSE).
