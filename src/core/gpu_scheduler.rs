// gpu_scheduler.rs
// BUG FIX: Fixed dead code — sched_cat was always "Games" in both restore and activate branches.
// Now correctly sets to "High Performance" when gaming, restores to "Games" on disable.

use winreg::enums::*;
use winreg::RegKey;
use log::info;

/// Windows Multimedia/Game task profile registry key
const GPU_SCHEDULER_KEY: &str = r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers";
const GAME_PROFILE_KEY: &str =
    r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games";

pub struct GpuScheduler {
    gaming_mode_active: bool,
    original_hw_sched: Option<u32>,
    original_gpu_priority: Option<u32>,
    original_priority: Option<u32>,
}

impl GpuScheduler {
    pub fn new() -> Self {
        Self {
            gaming_mode_active: false,
            original_hw_sched: None,
            original_gpu_priority: None,
            original_priority: None,
        }
    }

    pub fn enable_gaming_mode(&mut self) {
        if self.gaming_mode_active {
            return;
        }

        // Back up original values before modifying
        self.original_hw_sched = self.read_hags_value();
        self.original_gpu_priority = self.read_game_profile_value("GPU Priority");
        self.original_priority = self.read_game_profile_value("Priority");

        let _ = self.set_hags_value(2);
        // BUG FIX: Use "High Performance" scheduling category during gaming mode
        self.apply_game_profile(8, 2, "High Performance");

        info!("[GpuScheduler] Gaming mode enabled — GPU priority maximized.");
        self.gaming_mode_active = true;
    }

    pub fn disable_gaming_mode(&mut self) {
        if !self.gaming_mode_active {
            return;
        }

        if let Some(original) = self.original_hw_sched {
            let _ = self.set_hags_value(original);
        }

        // BUG FIX: Restore to original backed-up values, not hardcoded ones
        let gpu_prio = self.original_gpu_priority.unwrap_or(8);
        let prio = self.original_priority.unwrap_or(2);
        // BUG FIX: Restore to "Games" (original) not "High Performance"
        self.apply_game_profile(gpu_prio, prio, "Games");

        info!("[GpuScheduler] GPU scheduling restored to normal.");
        self.gaming_mode_active = false;
    }

    fn read_hags_value(&self) -> Option<u32> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        hklm.open_subkey(GPU_SCHEDULER_KEY)
            .ok()
            .and_then(|k| k.get_value::<u32, _>("HwSchMode").ok())
    }

    fn set_hags_value(&self, value: u32) -> std::io::Result<()> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let key = hklm.open_subkey_with_flags(GPU_SCHEDULER_KEY, KEY_ALL_ACCESS)?;
        key.set_value("HwSchMode", &value)?;
        Ok(())
    }

    fn read_game_profile_value(&self, name: &str) -> Option<u32> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        hklm.open_subkey(GAME_PROFILE_KEY)
            .ok()
            .and_then(|k| k.get_value::<u32, _>(name).ok())
    }

    fn apply_game_profile(&self, gpu_priority: u32, priority: u32, sched_cat: &str) {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(key) = hklm.open_subkey_with_flags(GAME_PROFILE_KEY, KEY_ALL_ACCESS) {
            let _ = key.set_value("GPU Priority", &gpu_priority);
            let _ = key.set_value("Priority", &priority);
            let _ = key.set_value("Scheduling Category", &sched_cat.to_string());
            info!("[GpuScheduler] Profile set: GPU={} Priority={} Cat={}", gpu_priority, priority, sched_cat);
        }
    }

    pub fn is_active(&self) -> bool {
        self.gaming_mode_active
    }
}

impl Drop for GpuScheduler {
    fn drop(&mut self) {
        self.disable_gaming_mode();
    }
}
