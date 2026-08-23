use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub timer_resolution_ms: f32,
    pub enable_high_precision_timer: bool,
    pub enable_cpu_affinity: bool,
    pub enable_power_plan_boost: bool,
    pub enable_gpu_boost: bool,
    pub enable_input_latency_tweaks: bool,
    pub enable_io_priority_tuning: bool,
    pub enable_standby_purging: bool,
    pub ram_warning_percent: f32,
    pub ram_critical_percent: f32,
    pub disk_auto_clean_percent: f32,
    pub explorer_memory_limit_mb: u64,
    pub autostart_on_boot: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timer_resolution_ms: 0.5,
            enable_high_precision_timer: true,
            enable_cpu_affinity: true,
            enable_power_plan_boost: true,
            enable_gpu_boost: true,
            enable_input_latency_tweaks: true,
            enable_io_priority_tuning: true,
            enable_standby_purging: true,
            ram_warning_percent: 80.0,
            ram_critical_percent: 90.0,
            disk_auto_clean_percent: 90.0,
            explorer_memory_limit_mb: 2500,
            autostart_on_boot: true,
        }
    }
}

impl Config {
    #[must_use] 
    pub fn get_config_path() -> PathBuf {
        if let Ok(exe) = std::env::current_exe() {
            let mut path = exe;
            path.set_extension("toml");
            path
        } else {
            PathBuf::from("smart-system-manager.toml")
        }
    }

    #[must_use] 
    pub fn load() -> Self {
        let path = Self::get_config_path();
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(config) = toml::from_str(&contents) {
                return config;
            }
        }
        let config = Self::default();
        let _ = config.save();
        config
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_config_path();
        let string = toml::to_string_pretty(self)?;
        fs::write(path, string)?;
        Ok(())
    }
}
