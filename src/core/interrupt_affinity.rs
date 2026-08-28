use log::warn;
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS, KEY_READ};
use winreg::RegKey;

pub struct InterruptAffinity;

impl InterruptAffinity {
    /// Forces hardware interrupts for Network and GPU adapters to specific CPU cores.
    /// This prevents "Core 0 bottleneck" where driver DPCs block the primary thread.
    pub fn optimize_interrupt_routing(
        p_core_mask: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if p_core_mask == 0 || p_core_mask == usize::MAX {
            return Ok(()); // Invalid or homogenous CPU
        }

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let pci_path = r"SYSTEM\CurrentControlSet\Enum\PCI";

        if let Ok(pci_key) = hklm.open_subkey_with_flags(pci_path, KEY_READ) {
            for device in pci_key.enum_keys().filter_map(Result::ok) {
                if let Ok(dev_key) = pci_key.open_subkey_with_flags(&device, KEY_READ) {
                    for instance in dev_key.enum_keys().filter_map(Result::ok) {
                        let inst_path = format!("{device}\\{instance}");
                        if let Ok(inst_key) = pci_key.open_subkey_with_flags(&inst_path, KEY_READ) {
                            let class_guid: String =
                                inst_key.get_value("ClassGUID").unwrap_or_default();

                            // Target GPU ({4D36E968-E325-11CE-BFC1-08002BE10318}) and NIC ({4D36E972-E325-11CE-BFC1-08002BE10318})
                            if class_guid
                                .eq_ignore_ascii_case("{4d36e968-e325-11ce-bfc1-08002be10318}")
                                || class_guid
                                    .eq_ignore_ascii_case("{4d36e972-e325-11ce-bfc1-08002be10318}")
                            {
                                let affinity_path = format!(
                                    "{inst_path}\\Device Parameters\\Interrupt Management\\Affinity Policy"
                                );

                                if let Ok((affinity_key, _)) =
                                    pci_key.create_subkey_with_flags(&affinity_path, KEY_ALL_ACCESS)
                                {
                                    // IrqPolicySpecifiedProcessors (0x04)
                                    let _ = affinity_key.set_value("DevicePolicy", &4u32);

                                    // Build the binary mask (AssignmentSetOverride is a REG_BINARY)
                                    let mask_bytes = (p_core_mask as u64).to_le_bytes();
                                    let reg_val = winreg::RegValue {
                                        vtype: winreg::enums::RegType::REG_BINARY,
                                        bytes: mask_bytes.to_vec(),
                                    };
                                    let _ = affinity_key
                                        .set_raw_value("AssignmentSetOverride", &reg_val);
                                } else {
                                    warn!("Failed to create Affinity Policy key for {inst_path}");
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
