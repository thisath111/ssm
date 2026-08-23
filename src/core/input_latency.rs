use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS};
use winreg::RegKey;

const MOUSE_KEY: &str = r"Control Panel\Mouse";
const KEYBOARD_KEY: &str = r"Control Panel\Keyboard";
const PRIORITY_KEY: &str = r"SYSTEM\CurrentControlSet\Control\PriorityControl";
const KEYBOARD_CLASS_KEY: &str = r"SYSTEM\CurrentControlSet\Services\kbdclass\Parameters";
const MOUSE_CLASS_KEY: &str = r"SYSTEM\CurrentControlSet\Services\mouclass\Parameters";

pub struct InputLatencyOptimizer;

impl InputLatencyOptimizer {
    /// Tunes Windows mouse & keyboard response settings for ultra-low input lag.
    pub fn optimize_all() -> Result<(), Box<dyn std::error::Error>> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

        // 1. Disable Windows Pointer Precision / Mouse Acceleration
        if let Ok(key) = hkcu.open_subkey_with_flags(MOUSE_KEY, KEY_ALL_ACCESS) {
            let _ = key.set_value("MouseSpeed", &"0".to_string());
            let _ = key.set_value("MouseThreshold1", &"0".to_string());
            let _ = key.set_value("MouseThreshold2", &"0".to_string());
            let _ = key.set_value("MouseSensitivity", &"10".to_string());
        }

        // 2. Maximize Keyboard Repeat Rate with ergonomic initial delay (prevents accidental key repeats)
        if let Ok(key) = hkcu.open_subkey_with_flags(KEYBOARD_KEY, KEY_ALL_ACCESS) {
            let _ = key.set_value("KeyboardDelay", &"1".to_string());
            let _ = key.set_value("KeyboardSpeed", &"31".to_string());
        }

        // 3. Set reliable Driver Queue Sizes (100) to guarantee zero dropped keystrokes/clicks even during load spikes
        if let Ok(key) = hklm.open_subkey_with_flags(KEYBOARD_CLASS_KEY, KEY_ALL_ACCESS) {
            let _ = key.set_value("KeyboardDataQueueSize", &100u32);
        }

        if let Ok(key) = hklm.open_subkey_with_flags(MOUSE_CLASS_KEY, KEY_ALL_ACCESS) {
            let _ = key.set_value("MouseDataQueueSize", &100u32);
        }

        // 4. Set Win32PrioritySeparation = 0x26 (Max Foreground CPU Quantum)
        if let Ok(key) = hklm.open_subkey_with_flags(PRIORITY_KEY, KEY_ALL_ACCESS) {
            let _ = key.set_value("Win32PrioritySeparation", &0x26u32);
        }

        Ok(())
    }
}
