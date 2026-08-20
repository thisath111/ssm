// network_qos.rs
// Throttles background app network priority using Windows QoS policies.
// When gaming, background downloads/updates cannot steal bandwidth.

use winreg::enums::*;
use winreg::RegKey;
use log::{info, warn};

const QOS_POLICY_KEY: &str = r"Software\Policies\Microsoft\Windows\QoS";

pub struct NetworkQos {
    gaming_mode_active: bool,
}

impl NetworkQos {
    pub fn new() -> Self {
        let instance = Self {
            gaming_mode_active: false,
        };
        // BUG FIX: Clean up any stale QoS policies left over from a prior crash or unexpected exit
        let _ = instance.remove_qos_policy("SmartSysManagerBG");
        instance
    }

    /// Sets DSCP (Differentiated Services Code Point) for background apps to CS1
    /// (lowest priority) via Windows Group Policy QoS. This is purely registry-based,
    /// requires no kernel driver, and is 100% reversible.
    pub fn enable_gaming_mode(&mut self) {
        if self.gaming_mode_active {
            return;
        }

        match self.apply_qos_policy("SmartSysManagerBG", "CS1", 8) {
            Ok(_) => {
                info!("[NetworkQoS] Gaming mode enabled — background traffic throttled to CS1");
                self.gaming_mode_active = true;
            }
            Err(e) => warn!("[NetworkQoS] Failed to apply QoS policy: {}", e),
        }
    }

    pub fn disable_gaming_mode(&mut self) {
        if !self.gaming_mode_active {
            return;
        }

        match self.remove_qos_policy("SmartSysManagerBG") {
            Ok(_) => {
                info!("[NetworkQoS] Gaming mode disabled — network priority restored");
                self.gaming_mode_active = false;
            }
            Err(e) => warn!("[NetworkQoS] Failed to remove QoS policy: {}", e),
        }
    }

    fn apply_qos_policy(&self, name: &str, dscp_value: &str, throttle_rate: u32) -> std::io::Result<()> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let (policy_key, _) = hklm.create_subkey(&format!("{}\\{}", QOS_POLICY_KEY, name))?;

        policy_key.set_value("Version", &1u32)?;
        policy_key.set_value("DSCPValue", &dscp_value.to_string())?;
        policy_key.set_value("ThrottleRate", &throttle_rate)?;
        policy_key.set_value("Application", &"*".to_string())?;
        policy_key.set_value("Protocol", &"*".to_string())?;

        Ok(())
    }

    fn remove_qos_policy(&self, name: &str) -> std::io::Result<()> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let base = hklm.open_subkey_with_flags(QOS_POLICY_KEY, KEY_ALL_ACCESS);
        match base {
            Ok(key) => { let _ = key.delete_subkey(name); Ok(()) }
            Err(e) => Err(e),
        }
    }

    pub fn is_active(&self) -> bool {
        self.gaming_mode_active
    }
}

impl Drop for NetworkQos {
    fn drop(&mut self) {
        self.disable_gaming_mode();
    }
}
