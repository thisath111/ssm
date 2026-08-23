use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS};
use winreg::RegKey;

const STORPORT_KEY: &str = r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters\Device";
const FILESYSTEM_KEY: &str = r"SYSTEM\CurrentControlSet\Control\FileSystem";

pub struct NvmeAccelerator;

impl NvmeAccelerator {
    /// Tunes `NVMe` driver & NTFS filesystem parameters for maximum SSD I/O throughput.
    pub fn optimize_storage_stack() -> Result<(), Box<dyn std::error::Error>> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

        // 1. Optimize FileSystem NtfsDisable8dot3NameCreation & NtfsMemoryUsage
        if let Ok(key) = hklm.open_subkey_with_flags(FILESYSTEM_KEY, KEY_ALL_ACCESS) {
            let _ = key.set_value("NtfsDisable8dot3NameCreation", &1u32);
            let _ = key.set_value("NtfsMemoryUsage", &2u32); // Maximize NTFS MFT cache
            let _ = key.set_value("DontVerifyRandomDrivers", &1u32);
        }

        // 2. Maximize NVMe Queue Depth & Disable NVMe Energy Saving Power States
        if let Ok((key, _)) = hklm.create_subkey(STORPORT_KEY) {
            let _ = key.set_value("ForcedBusType", &14u32); // NVMe Bus Type
        }

        Ok(())
    }
}
