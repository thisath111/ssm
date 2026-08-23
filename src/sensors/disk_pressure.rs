use std::path::PathBuf;
use windows::core::HSTRING;
use windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;

pub struct DiskPressureSensor {
    pub usage_percent: f32,
    pub free_gb: f64,
    pub total_gb: f64,
}

impl Default for DiskPressureSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl DiskPressureSensor {
    #[must_use] 
    pub fn new() -> Self {
        let mut sensor = Self {
            usage_percent: 0.0,
            free_gb: 0.0,
            total_gb: 0.0,
        };
        sensor.update();
        sensor
    }

    pub fn update(&mut self) {
        let drive = HSTRING::from("C:\\");
        let mut free_bytes: u64 = 0;
        let mut total_bytes: u64 = 0;
        let mut _total_free: u64 = 0;

        let ok = unsafe {
            GetDiskFreeSpaceExW(
                &drive,
                Some(&raw mut free_bytes),
                Some(&raw mut total_bytes),
                Some(&raw mut _total_free),
            )
        };

        if ok.is_ok() && total_bytes > 0 {
            let used = total_bytes.saturating_sub(free_bytes);
            self.usage_percent = (used as f64 / total_bytes as f64 * 100.0) as f32;
            self.free_gb = free_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
            self.total_gb = total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        }
    }

    #[must_use] 
    pub fn clean_temp_files(&self) -> (u64, u32) {
        let dirs_to_clean = vec![
            std::env::temp_dir(),
            PathBuf::from(r"C:\Windows\Temp"),
            std::env::var("LOCALAPPDATA").map_or_else(|_| std::env::temp_dir(), |p| PathBuf::from(p).join("Temp")),
        ];

        let mut cleaned_bytes: u64 = 0;
        let mut cleaned_files: u32 = 0;

        for dir in dirs_to_clean {
            if !dir.exists() {
                continue;
            }
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Ok(metadata) = path.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            if let Ok(age) = modified.elapsed() {
                                if age.as_secs() < 43200 {
                                    continue;
                                }
                            }
                        }

                        let size = metadata.len();
                        if path.is_file() {
                            if std::fs::remove_file(&path).is_ok() {
                                cleaned_bytes += size;
                                cleaned_files += 1;
                            }
                        } else if path.is_dir()
                            && std::fs::remove_dir_all(&path).is_ok() {
                                cleaned_bytes += size;
                                cleaned_files += 1;
                            }
                    }
                }
            }
        }

        (cleaned_bytes, cleaned_files)
    }
}
