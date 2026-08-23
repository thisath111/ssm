use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RamPressureLevel {
    Normal,
    Warning,
    Critical,
    Emergency,
}

pub struct RamPressureSensor {
    pub level: RamPressureLevel,
    pub usage_percent: f32,
    pub available_mb: u64,
    pub total_mb: u64,
}

impl Default for RamPressureSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl RamPressureSensor {
    #[must_use] 
    pub fn new() -> Self {
        let mut sensor = Self {
            level: RamPressureLevel::Normal,
            usage_percent: 0.0,
            available_mb: 0,
            total_mb: 0,
        };
        sensor.update();
        sensor
    }

    pub fn update(&mut self) {
        let mut mem_info = MEMORYSTATUSEX {
            dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
            ..Default::default()
        };

        if unsafe { GlobalMemoryStatusEx(&raw mut mem_info) }.is_ok() {
            self.total_mb = mem_info.ullTotalPhys / (1024 * 1024);
            self.available_mb = mem_info.ullAvailPhys / (1024 * 1024);
            let used_mb = self.total_mb.saturating_sub(self.available_mb);
            self.usage_percent = if self.total_mb > 0 {
                (used_mb as f32 / self.total_mb as f32) * 100.0
            } else {
                0.0
            };

            self.level = if self.usage_percent >= 95.0 || self.available_mb < 500 {
                RamPressureLevel::Emergency
            } else if self.usage_percent >= 90.0 || self.available_mb < 1500 {
                RamPressureLevel::Critical
            } else if self.usage_percent >= 80.0 {
                RamPressureLevel::Warning
            } else {
                RamPressureLevel::Normal
            };
        }
    }
}
