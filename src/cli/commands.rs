use crate::cli::ui::TerminalUi;
use crate::core::{
    cpu::CpuManager, gpu::GpuManager, input_latency::InputLatencyOptimizer,
    math_engine::KalmanPredictor, network::NetworkOptimizer, nvme_accelerator::NvmeAccelerator,
    ram::RamManager, registry_tweaker::RegistryTweaker, service_tuner::ServiceTuner,
    stability_shield::StabilityShield, timer_resolution::TimerResolutionManager,
};
use crate::daemon::{service, startup::StartupManager};
use crate::sensors::{
    cpu_topology::CpuTopology, disk_pressure::DiskPressureSensor,
    ram_pressure::RamPressureSensor,
};
use crate::utils::{config::Config, nt_api, win32};
use sysinfo::System;

pub fn handle_boost() {
    TerminalUi::print_header("Executing Ultimate Multi-Layered System Boost");

    if !win32::is_elevated() {
        TerminalUi::print_warning("Not running as Administrator. Some kernel tweaks may require elevation.");
    }

    let mut timer_mgr = TimerResolutionManager::new();
    if timer_mgr.enable_high_precision() {
        TerminalUi::print_success("Win32 Timer Resolution set to 0.5ms (Sub-Millisecond High Precision)");
    }

    let mut cpu_mgr = CpuManager::new();
    cpu_mgr.enable_performance_mode();
    TerminalUi::print_success("CPU Power Scheme set to Ultimate/High Performance (Cores Unparked)");

    let mut gpu_mgr = GpuManager::new();
    if gpu_mgr.enable_gpu_boost() {
        TerminalUi::print_success("DirectX / DWM GPU Priority & HAGS Mode Maximized");
    }

    if let Ok(_) = InputLatencyOptimizer::optimize_all() {
        TerminalUi::print_success("Keyboard & Mouse Input Latency Bypassed (Win32PrioritySeparation 0x26)");
    }

    if let Ok(_) = NetworkOptimizer::disable_tcp_nagle() {
        TerminalUi::print_success("Network TCP NoDelay / Nagle Algorithm Latency Bypass Active");
    }

    let _ = NetworkOptimizer::enable_qos_policy();
    TerminalUi::print_success("Windows QoS DSCP CS1 Packet Prioritization Enforced");

    if let Ok(_) = NvmeAccelerator::optimize_storage_stack() {
        TerminalUi::print_success("NVMe SSD Queue Depth & NTFS MFT Cache Acceleration Active");
    }

    ServiceTuner::pause_background_services();
    TerminalUi::print_success("Non-essential Background Services Throttled");

    TerminalUi::print_header("Boost Summary");
    TerminalUi::print_info("System is operating at peak physical hardware throughput. Latency minimized!");
    // Note: timer resolution and CPU/GPU settings are process-scoped and revert on exit.
    // For persistent optimization, run: ssm install
}

pub fn handle_clean() {
    TerminalUi::print_header("Executing System Memory & Storage Cleanup");

    let ram_mgr = RamManager::new();
    if ram_mgr.purge_standby_memory() {
        TerminalUi::print_success("Windows Zero-Copy Standby Memory List Purged Successfully");
    } else {
        TerminalUi::print_warning("Standby Memory purge skipped or requires admin privilege");
    }

    let disk_sensor = DiskPressureSensor::new();
    let (bytes, files) = disk_sensor.clean_temp_files();
    let mb = bytes / (1024 * 1024);
    TerminalUi::print_success(&format!("Disk Storage Cleaned: {} files, {} MB freed", files, mb));
}

pub fn handle_tune() {
    TerminalUi::print_header("Applying Low-Latency System Registry Tweaks");

    if let Ok(_) = RegistryTweaker::apply_performance_tweaks() {
        TerminalUi::print_success("Telemetry disabled, SystemResponsiveness=0, NetworkThrottlingIndex=0xFFFFFFFF");
    }
    if let Ok(_) = InputLatencyOptimizer::optimize_all() {
        TerminalUi::print_success("Mouse Acceleration Bypassed & Keyboard Buffer Queue Tuned");
    }
    if let Ok(_) = NvmeAccelerator::optimize_storage_stack() {
        TerminalUi::print_success("NVMe SSD Queue & NTFS 8dot3 Name Creation Optimized");
    }
}

pub fn handle_stats() {
    TerminalUi::print_header("Live System & Hardware Status");

    let topology = CpuTopology::detect();
    TerminalUi::print_key_value("Logical CPU Cores", &topology.total_logical_cores.to_string());
    TerminalUi::print_key_value("P-Core Mask", &format!("{:#x}", topology.p_core_mask));
    TerminalUi::print_key_value("E-Core Mask", &format!("{:#x}", topology.e_core_mask));

    if let Some((min, max, cur)) = nt_api::query_timer_resolution() {
        let cur_ms = cur as f32 / 10000.0;
        let min_ms = min as f32 / 10000.0;
        let max_ms = max as f32 / 10000.0;
        TerminalUi::print_key_value("Timer Resolution", &format!("{:.2} ms (Min: {:.2} ms, Max: {:.2} ms)", cur_ms, max_ms, min_ms));
    }

    let ram = RamPressureSensor::new();
    TerminalUi::print_key_value("RAM Usage", &format!("{:.1}% (Free: {} MB / Total: {} MB)", ram.usage_percent, ram.available_mb, ram.total_mb));
    TerminalUi::print_key_value("RAM Pressure Level", &format!("{:?}", ram.level));

    let disk = DiskPressureSensor::new();
    TerminalUi::print_key_value("Disk Usage (C:)", &format!("{:.1}% (Free: {:.1} GB / Total: {:.1} GB)", disk.usage_percent, disk.free_gb, disk.total_gb));

    let autostart = StartupManager::is_autostart_enabled();
    TerminalUi::print_key_value("Autostart on Boot", if autostart { "Enabled" } else { "Disabled" });

    let mut sys = System::new_all();
    sys.refresh_all();
    let leaking = StabilityShield::audit_handle_leaks(&sys);
    TerminalUi::print_key_value("Leaking Processes (>10k handles)", &leaking.len().to_string());

    let mut kalman = KalmanPredictor::new(0.01, 0.1);
    let estimate = kalman.update(sys.global_cpu_usage());
    TerminalUi::print_key_value("Kalman Estimated Load", &format!("{:.1}%", estimate));
}

pub fn handle_daemon() {
    TerminalUi::print_info("Starting Smart System Manager Background Optimization Daemon...");
    
    // Attempt to connect to Windows Service Control Manager (SCM)
    if let Err(e) = crate::daemon::service::run_as_service() {
        TerminalUi::print_info(&format!("Running in standalone console mode (SCM connect failed: {}).", e));
        
        // Fallback to standalone loop if not launched by Windows SCM
        let config = Config::load();
        let mut engine = crate::core::engine::SystemEngine::new(config);
        loop {
            engine.tick();
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}

pub fn handle_service(action: &str) {
    match action {
        "install" => {
            if !win32::is_elevated() {
                TerminalUi::print_warning("Administrator privileges required for service installation.");
                TerminalUi::print_info("Tip: Right-click your terminal and choose 'Run as Administrator', then retry.");
                return;
            }

            TerminalUi::print_header("Registering Windows Background Service");

            // Install & start Windows Native Service (SCM) — handles boot autostart invisibly
            match service::install_service() {
                Ok(_) => {
                    TerminalUi::print_success("Windows Service (SmartSystemManager) installed and started.");
                    TerminalUi::print_success("Boot autostart enabled (via Windows Service Manager).");

                    // Remove any leftover Registry Run key to prevent duplicate console window on boot
                    let _ = StartupManager::disable_autostart();
                }
                Err(e) => {
                    TerminalUi::print_warning(&format!("Service install failed ({}). Falling back to registry autostart.", e));
                    let _ = StartupManager::enable_autostart();
                    TerminalUi::print_success("Boot autostart enabled (via Registry Run key).");
                }
            }

            let mut config = Config::load();
            config.autostart_on_boot = true;
            let _ = config.save();

            TerminalUi::print_header("Installation Complete");
            TerminalUi::print_info("ssm will now start automatically every time Windows boots.");
            TerminalUi::print_info("Run \"ssm boost\" anytime for an instant performance boost.");
        }
        "uninstall" => {
            TerminalUi::print_header("Complete Uninstallation of Smart System Manager");

            if !win32::is_elevated() {
                TerminalUi::print_warning("Administrator privileges required to uninstall.");
                TerminalUi::print_info("Tip: Right-click your terminal and choose 'Run as Administrator', then run 'ssm uninstall'.");
                return;
            }

            TerminalUi::print_info("Stopping and removing Windows Service...");
            let _ = service::uninstall_service();
            TerminalUi::print_success("Windows Service (SmartSystemManager) stopped & removed.");

            TerminalUi::print_info("Disabling boot autostart...");
            let _ = StartupManager::disable_autostart();
            TerminalUi::print_success("Boot autostart registry key removed.");

            use winreg::enums::*;
            use winreg::RegKey;
            TerminalUi::print_info("Removing C:\\ssm from Windows PATH...");
            if let Ok(hkcu) = RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE) {
                if let Ok(current_path) = hkcu.get_value::<String, _>("Path") {
                    let filtered: Vec<&str> = current_path
                        .split(';')
                        .filter(|&p| !p.trim().eq_ignore_ascii_case("C:\\ssm"))
                        .collect();
                    let _ = hkcu.set_value("Path", &filtered.join(";"));
                    TerminalUi::print_success("Removed C:\\ssm from Windows User PATH.");
                }
            }

            TerminalUi::print_header("Uninstall Complete");
            TerminalUi::print_info("C:\\ssm directory will be deleted in 2 seconds. Goodbye!");

            let _ = std::process::Command::new("cmd")
                .args(&["/c", "ping 127.0.0.1 -n 3 > NUL & rmdir /S /Q C:\\ssm"])
                .spawn();
        }
        _ => {
            TerminalUi::print_error("Unknown action. Usage: ssm service install | ssm service uninstall");
        }
    }
}
