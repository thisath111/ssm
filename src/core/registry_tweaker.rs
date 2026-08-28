use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS, KEY_READ};
use winreg::RegKey;

pub struct RegistryTweaker;

impl RegistryTweaker {
    /// Applies zero-latency, high-performance system registry tweaks.
    pub fn apply_performance_tweaks() -> Result<(), Box<dyn std::error::Error>> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

        // 1. Disable Network Throttling & Reserve Bandwidth Limit
        let sys_profile_key =
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile";
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
            for device in pci_key.enum_keys().filter_map(std::result::Result::ok) {
                if let Ok(dev_key) = pci_key.open_subkey_with_flags(&device, KEY_READ) {
                    for instance in dev_key.enum_keys().filter_map(std::result::Result::ok) {
                        let inst_path = format!("{device}\\{instance}");
                        if let Ok(inst_key) = pci_key.open_subkey_with_flags(&inst_path, KEY_READ) {
                            let class_guid: String =
                                inst_key.get_value("ClassGUID").unwrap_or_default();
                            // GPU: {4D36E968-E325-11CE-BFC1-08002BE10318}
                            // Network: {4D36E972-E325-11CE-BFC1-08002BE10318}
                            if class_guid
                                .eq_ignore_ascii_case("{4d36e968-e325-11ce-bfc1-08002be10318}")
                                || class_guid
                                    .eq_ignore_ascii_case("{4d36e972-e325-11ce-bfc1-08002be10318}")
                            {
                                let msi_path = format!(
                                    "{}\\{}",
                                    inst_path,
                                    r"Device Parameters\Interrupt Management\MessageSignaledInterruptProperties"
                                );
                                if let Ok((msi_key, _)) =
                                    pci_key.create_subkey_with_flags(&msi_path, KEY_ALL_ACCESS)
                                {
                                    let _ = msi_key.set_value("MSISupported", &1u32);
                                }
                            }
                        }
                    }
                }
            }
        }

        // 5. Fast Shutdown & Auto-End Tasks
        let control_key = r"SYSTEM\CurrentControlSet\Control";
        if let Ok((key, _)) = hklm.create_subkey_with_flags(control_key, KEY_ALL_ACCESS) {
            let _ = key.set_value("WaitToKillServiceTimeout", &"2000");
        }

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let desktop_key = r"Control Panel\Desktop";
        if let Ok((key, _)) = hkcu.create_subkey_with_flags(desktop_key, KEY_ALL_ACCESS) {
            let _ = key.set_value("AutoEndTasks", &"1");
            let _ = key.set_value("WaitToKillAppTimeout", &"2000");
            let _ = key.set_value("HungAppTimeout", &"2000");
            let _ = key.set_value("MenuShowDelay", &"0"); // Instant menu popup
        }

        // 6. Optimize Prefetch & Superfetch for faster boot
        let prefetch_key = r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters";
        if let Ok(key) = hklm.open_subkey_with_flags(prefetch_key, KEY_ALL_ACCESS) {
            let _ = key.set_value("EnablePrefetcher", &2u32); // Boot files only
            let _ = key.set_value("EnableSuperfetch", &0u32); // Disable Superfetch (prevents disk thrashing)
        }

        // 7. Memory Management: LargeSystemCache & DisablePagingExecutive
        let mm_key = r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management";
        if let Ok(key) = hklm.open_subkey_with_flags(mm_key, KEY_ALL_ACCESS) {
            let _ = key.set_value("LargeSystemCache", &1u32); // More RAM for file cache
            let _ = key.set_value("DisablePagingExecutive", &1u32); // Keep kernel in RAM
            let _ = key.set_value("ClearPageFileAtShutdown", &0u32); // Faster shutdown
        }

        // 8. NTFS: Disable Last Access Time stamp (major I/O reduction)
        let filesystem_key = r"SYSTEM\CurrentControlSet\Control\FileSystem";
        if let Ok(key) = hklm.open_subkey_with_flags(filesystem_key, KEY_ALL_ACCESS) {
            let _ = key.set_value("NtfsDisableLastAccessUpdate", &0x80000003u32);
        }

        // 9. Reduce Visual Effects for snappier UI (disable animations/transparency)
        let visual_key = r"Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects";
        if let Ok((key, _)) = hkcu.create_subkey_with_flags(visual_key, KEY_ALL_ACCESS) {
            let _ = key.set_value("VisualFXSetting", &2u32); // Custom
        }
        let adv_key = r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced";
        if let Ok(key) = hkcu.open_subkey_with_flags(adv_key, KEY_ALL_ACCESS) {
            let _ = key.set_value("TaskbarAnimations", &0u32);
        }
        let dwm_key = r"Software\Microsoft\Windows\DWM";
        if let Ok(key) = hkcu.open_subkey_with_flags(dwm_key, KEY_ALL_ACCESS) {
            let _ = key.set_value("EnableAeroPeek", &0u32);
            let _ = key.set_value("AlwaysHibernateThumbnails", &0u32);
        }

        // 10. Disable Windows Tips & Suggestions (background CPU drain)
        let content_key = r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager";
        if let Ok(key) = hkcu.open_subkey_with_flags(content_key, KEY_ALL_ACCESS) {
            let _ = key.set_value("SubscribedContent-338389Enabled", &0u32);
            let _ = key.set_value("SubscribedContent-310093Enabled", &0u32);
            let _ = key.set_value("SystemPaneSuggestionsEnabled", &0u32);
            let _ = key.set_value("SoftLandingEnabled", &0u32);
        }

        // 11. Disable Game Bar & Game DVR (massive background overhead)
        let game_dvr_key = r"Software\Microsoft\Windows\CurrentVersion\GameDVR";
        if let Ok((key, _)) = hkcu.create_subkey_with_flags(game_dvr_key, KEY_ALL_ACCESS) {
            let _ = key.set_value("AppCaptureEnabled", &0u32);
        }
        let game_bar_key = r"Software\Microsoft\GameBar";
        if let Ok((key, _)) = hkcu.create_subkey_with_flags(game_bar_key, KEY_ALL_ACCESS) {
            let _ = key.set_value("AllowAutoGameMode", &1u32);
            let _ = key.set_value("AutoGameModeEnabled", &1u32);
        }

        // 12. Global Timer Resolution Override (Windows 11 Fix)
        let sm_kernel_key = r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel";
        if let Ok(key) = hklm.open_subkey_with_flags(sm_kernel_key, KEY_ALL_ACCESS) {
            let _ = key.set_value("GlobalTimerResolutionRequests", &1u32);
        }

        // 13. CPU Priority Separation (Favor Foreground Apps)
        let prio_key = r"SYSTEM\CurrentControlSet\Control\PriorityControl";
        if let Ok(key) = hklm.open_subkey_with_flags(prio_key, KEY_ALL_ACCESS) {
            let _ = key.set_value("Win32PrioritySeparation", &0x26u32);
        }

        // 14. Disable USB Selective Suspend (Reduces Input Latency)
        let usb_key = r"SYSTEM\CurrentControlSet\Services\USB";
        if let Ok((key, _)) = hklm.create_subkey_with_flags(usb_key, KEY_ALL_ACCESS) {
            let _ = key.set_value("DisableSelectiveSuspend", &1u32);
        }

        // 15. Windows Defender Self-Exclusion (Prevents scanning our own daemon mid-operation)
        if let Ok(exe_path) = std::env::current_exe() {
            let def_key = r"SOFTWARE\Microsoft\Windows Defender\Exclusions\Paths";
            // Note: This requires high privileges. It might fail if Tamper Protection is on, which is fine.
            if let Ok((key, _)) = hklm.create_subkey_with_flags(def_key, KEY_ALL_ACCESS) {
                let _ = key.set_value(exe_path.to_string_lossy().as_ref(), &0u32);
            }
        }

        Ok(())
    }
}
