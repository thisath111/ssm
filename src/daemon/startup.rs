use winreg::enums::{HKEY_CURRENT_USER, KEY_ALL_ACCESS};
use winreg::RegKey;

const AUTOSTART_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const APP_NAME: &str = "SmartSystemManager";

pub struct StartupManager;

impl StartupManager {
    /// Enables auto-start on Windows user login.
    pub fn enable_autostart() -> Result<(), Box<dyn std::error::Error>> {
        let exe_path = std::env::current_exe()?;
        let exe_str = format!("\"{}\" daemon", exe_path.to_str().unwrap());

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (key, _) = hkcu.create_subkey(AUTOSTART_KEY)?;
        key.set_value(APP_NAME, &exe_str)?;
        Ok(())
    }

    /// Disables auto-start on Windows user login.
    pub fn disable_autostart() -> Result<(), Box<dyn std::error::Error>> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(key) = hkcu.open_subkey_with_flags(AUTOSTART_KEY, KEY_ALL_ACCESS) {
            let _ = key.delete_value(APP_NAME);
        }
        Ok(())
    }

    /// Checks if autostart registry entry is currently active.
    pub fn is_autostart_enabled() -> bool {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(key) = hkcu.open_subkey(AUTOSTART_KEY) {
            key.get_value::<String, _>(APP_NAME).is_ok()
        } else {
            false
        }
    }
}
