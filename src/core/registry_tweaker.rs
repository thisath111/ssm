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

        // 4. Force MSI (Message Signaled Interrupts) Mode for GPU/Network to lower DPC Latency
        let pci_path = r"SYSTEM\CurrentControlSet\Enum\PCI";
        if let Ok(pci_key) = hklm.open_subkey_with_flags(pci_path, KEY_READ) {
            for device in pci_key.enum_keys().filter_map(|k| k.ok()) {
                if let Ok(dev_key) = pci_key.open_subkey_with_flags(&device, KEY_READ) {
                    for instance in dev_key.enum_keys().filter_map(|k| k.ok()) {
                        let inst_path = format!("{}\\{}", device, instance);
                        if let Ok(inst_key) = pci_key.open_subkey_with_flags(&inst_path, KEY_READ) {
                            let class_guid: String = inst_key.get_value("ClassGUID").unwrap_or_default();
                            // GPU: {4D36E968-E325-11CE-BFC1-08002BE10318}
                            // Network: {4D36E972-E325-11CE-BFC1-08002BE10318}
                            if class_guid.eq_ignore_ascii_case("{4d36e968-e325-11ce-bfc1-08002be10318}") ||
                               class_guid.eq_ignore_ascii_case("{4d36e972-e325-11ce-bfc1-08002be10318}") {
                                
                                let msi_path = format!("{}\\{}", inst_path, r"Device Parameters\Interrupt Management\MessageSignaledInterruptProperties");
                                if let Ok((msi_key, _)) = pci_key.create_subkey_with_flags(&msi_path, KEY_ALL_ACCESS) {
                                    let _ = msi_key.set_value("MSISupported", &1u32);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
