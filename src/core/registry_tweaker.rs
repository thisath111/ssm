use winreg::enums::*;
use winreg::RegKey;

pub struct RegistryTweaker;

impl RegistryTweaker {
    /// Applies zero-latency, high-performance system registry tweaks.
    pub fn apply_performance_tweaks() -> Result<(), Box<dyn std::error::Error>> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

        // 1. Disable Network Throttling & Reserve Bandwidth Limit
        let sys_profile_key = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile";
        if let Ok(key) = hklm.open_subkey_with_flags(sys_profile_key, KEY_ALL_ACCESS) {
            let _ = key.set_value("NetworkThrottlingIndex", &0xFFFFFFFFu32);
            let _ = key.set_value("SystemResponsiveness", &0u32);
        }

        // 2. Disable Microsoft Telemetry Policies
        let telemetry_key = r"SOFTWARE\Policies\Microsoft\Windows\DataCollection";
        if let Ok((key, _)) = hklm.create_subkey(telemetry_key) {
            let _ = key.set_value("AllowTelemetry", &0u32);
        }

        // 3. Disable Cortana & Search Indexer Spikes during Gaming
        let search_key = r"SOFTWARE\Policies\Microsoft\Windows\Windows Search";
        if let Ok((key, _)) = hklm.create_subkey(search_key) {
            let _ = key.set_value("AllowCortana", &0u32);
            let _ = key.set_value("DisableSearchBoxSuggestions", &1u32);
        }

        Ok(())
    }
}
