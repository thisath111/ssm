use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS};
use winreg::RegKey;

const GPU_SCHEDULER_KEY: &str = r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers";
const GAME_PROFILE_KEY: &str =
    r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games";

pub struct GpuManager {
    is_boosted: bool,
    original_hags: Option<u32>,
    original_gpu_priority: Option<u32>,
}

impl Default for GpuManager {
    fn default() -> Self {
        Self::new()
    }
}

impl GpuManager {
    #[must_use] 
    pub const fn new() -> Self {
        Self {
            is_boosted: false,
            original_hags: None,
            original_gpu_priority: None,
        }
    }

    pub fn enable_gpu_boost(&mut self) -> bool {
        if self.is_boosted {
            return true;
        }

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

        // Backup & Enable Hardware-Accelerated GPU Scheduling (HAGS = 2)
        if let Ok(key) = hklm.open_subkey_with_flags(GPU_SCHEDULER_KEY, KEY_ALL_ACCESS) {
            if let Ok(val) = key.get_value::<u32, _>("HwSchMode") {
                self.original_hags = Some(val);
            }
            let _ = key.set_value("HwSchMode", &2u32);
        }

        // Optimize Games Multimedia Profile Registry Keys
        if let Ok(key) = hklm.open_subkey_with_flags(GAME_PROFILE_KEY, KEY_ALL_ACCESS) {
            if let Ok(val) = key.get_value::<u32, _>("GPU Priority") {
                self.original_gpu_priority = Some(val);
            }
            let _ = key.set_value("GPU Priority", &8u32);
            let _ = key.set_value("Priority", &6u32);
            let _ = key.set_value("Scheduling Category", &"High Performance".to_string());
            let _ = key.set_value("SFIO Priority", &"High".to_string());
        }

        self.is_boosted = true;
        true
    }

    pub fn restore_default(&mut self) {
        if !self.is_boosted {
            return;
        }

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Some(orig_hags) = self.original_hags {
            if let Ok(key) = hklm.open_subkey_with_flags(GPU_SCHEDULER_KEY, KEY_ALL_ACCESS) {
                let _ = key.set_value("HwSchMode", &orig_hags);
            }
        }

        if let Ok(key) = hklm.open_subkey_with_flags(GAME_PROFILE_KEY, KEY_ALL_ACCESS) {
            let orig_prio = self.original_gpu_priority.unwrap_or(8);
            let _ = key.set_value("GPU Priority", &orig_prio);
            let _ = key.set_value("Priority", &2u32);
            let _ = key.set_value("Scheduling Category", &"Games".to_string());
            let _ = key.set_value("SFIO Priority", &"Normal".to_string());
        }

        self.is_boosted = false;
    }
}

impl Drop for GpuManager {
    fn drop(&mut self) {
        self.restore_default();
    }
}
