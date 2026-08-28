use crate::core::ram::RamManager;
use sysinfo::System;
use windows::Win32::UI::WindowsAndMessaging::{GetShellWindow, IsHungAppWindow};

pub struct ExplorerWatchdog {
    last_check_tick: u64,
}

impl Default for ExplorerWatchdog {
    fn default() -> Self {
        Self::new()
    }
}

impl ExplorerWatchdog {
    #[must_use]
    pub const fn new() -> Self {
        Self { last_check_tick: 0 }
    }

    pub fn check(&mut self, sys: &System, ram_mgr: &RamManager, limit_mb: u64, tick_count: u64) {
        if tick_count.saturating_sub(self.last_check_tick) < 10 {
            return;
        }
        self.last_check_tick = tick_count;

        // Auto-Rescue: Restart Explorer if hung
        if Self::is_shell_hung() {
            let _ = std::process::Command::new("taskkill")
                .args(["/F", "/IM", "explorer.exe"])
                .spawn();
            let _ = std::process::Command::new("cmd")
                .args(["/c", "start explorer.exe"])
                .spawn();
            return;
        }

        let limit_bytes = limit_mb * 1024 * 1024;
        for (pid, process) in sys.processes() {
            let name = process.name().to_string_lossy().to_lowercase();
            if name == "explorer.exe" && process.memory() > limit_bytes {
                let _ = ram_mgr.trim_single_process(pid.as_u32());
            }
        }
    }

    #[must_use]
    pub fn is_shell_hung() -> bool {
        unsafe {
            let shell_hwnd = GetShellWindow();
            if shell_hwnd.is_invalid() || shell_hwnd.0.is_null() {
                false
            } else {
                IsHungAppWindow(shell_hwnd).as_bool()
            }
        }
    }
}
