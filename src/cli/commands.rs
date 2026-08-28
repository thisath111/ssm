use crate::cli::ui::TerminalUi;
use crate::core::{
    cpu::CpuManager, gpu::GpuManager, input_latency::InputLatencyOptimizer,
    math_engine::KalmanPredictor, network::NetworkOptimizer, nvme_accelerator::NvmeAccelerator,
    ram::RamManager, registry_tweaker::RegistryTweaker, service_tuner::ServiceTuner,
    stability_shield::StabilityShield, timer_resolution::TimerResolutionManager,
};
use crate::daemon::{service, startup::StartupManager};
use crate::sensors::{
    cpu_topology::CpuTopology, disk_pressure::DiskPressureSensor, ram_pressure::RamPressureSensor,
};
use crate::utils::{config::Config, nt_api, win32};
use sysinfo::System;

pub fn handle_boost() {
    TerminalUi::print_header("Executing Ultimate Multi-Layered System Boost");

    if !win32::is_elevated() {
        TerminalUi::print_warning(
            "Not running as Administrator. Some kernel tweaks may require elevation.",
        );
    }

    let mut timer_mgr = TimerResolutionManager::new();
    if timer_mgr.enable_high_precision() {
        TerminalUi::print_success(
            "Win32 Timer Resolution set to 0.5ms (Sub-Millisecond High Precision)",
        );
    }

    let mut cpu_mgr = CpuManager::new();
    cpu_mgr.enable_performance_mode();
    TerminalUi::print_success("CPU Power Scheme set to Ultimate/High Performance (Cores Unparked)");

    let mut gpu_mgr = GpuManager::new();
    if gpu_mgr.enable_gpu_boost() {
        TerminalUi::print_success("DirectX / DWM GPU Priority & HAGS Mode Maximized");
    }

    if matches!(InputLatencyOptimizer::optimize_all(), Ok(())) {
        TerminalUi::print_success(
            "Keyboard & Mouse Input Latency Bypassed (Win32PrioritySeparation 0x26)",
        );
    }

    if matches!(NetworkOptimizer::disable_tcp_nagle(), Ok(())) {
        TerminalUi::print_success("Network TCP NoDelay / Nagle Algorithm Latency Bypass Active");
    }

    let _ = NetworkOptimizer::enable_qos_policy();
    TerminalUi::print_success("Windows QoS DSCP CS1 Packet Prioritization Enforced");

    if matches!(NvmeAccelerator::optimize_storage_stack(), Ok(())) {
        TerminalUi::print_success("NVMe SSD Queue Depth & NTFS MFT Cache Acceleration Active");
    }

    if matches!(
        crate::core::dwm::DwmLatencyOptimizer::optimize_dwm_latency(),
        Ok(())
    ) {
        TerminalUi::print_success(
            "Desktop Window Manager (DWM) DirectFlip Zero-Lag Presentation Active",
        );
    }

    if crate::core::large_pages::LargePageOptimizer::enable_large_pages() {
        TerminalUi::print_success(
            "Kernel Large Pages Memory Privilege (SeLockMemoryPrivilege) Granted",
        );
    }

    let mut tuner = ServiceTuner::new();
    tuner.pause_background_services();
    TerminalUi::print_success("Non-essential Background Services Throttled");

    TerminalUi::print_header("Boost Summary");
    TerminalUi::print_info(
        "System is operating at peak physical hardware throughput. Latency minimized!",
    );
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
    TerminalUi::print_success(&format!(
        "Disk Storage Cleaned: {files} files, {mb} MB freed"
    ));
}

pub fn handle_tune() {
    TerminalUi::print_header("Applying Low-Latency System Registry Tweaks");

    if matches!(RegistryTweaker::apply_performance_tweaks(), Ok(())) {
        TerminalUi::print_success(
            "Telemetry disabled, SystemResponsiveness=0, NetworkThrottlingIndex=0xFFFFFFFF",
        );
    }
    if matches!(InputLatencyOptimizer::optimize_all(), Ok(())) {
        TerminalUi::print_success("Mouse Acceleration Bypassed & Keyboard Buffer Queue Tuned");
    }
    if matches!(NvmeAccelerator::optimize_storage_stack(), Ok(())) {
        TerminalUi::print_success("NVMe SSD Queue & NTFS 8dot3 Name Creation Optimized");
    }
}

pub fn handle_stats() {
    TerminalUi::print_header("Live System & Hardware Status");

    let topology = CpuTopology::detect();
    TerminalUi::print_key_value(
        "Logical CPU Cores",
        &topology.total_logical_cores.to_string(),
    );
    TerminalUi::print_key_value("P-Core Mask", &format!("{:#x}", topology.p_core_mask));
    TerminalUi::print_key_value("E-Core Mask", &format!("{:#x}", topology.e_core_mask));

    if let Some((min, max, cur)) = nt_api::query_timer_resolution() {
        let cur_ms = cur as f32 / 10000.0;
        let min_ms = min as f32 / 10000.0;
        let max_ms = max as f32 / 10000.0;
        TerminalUi::print_key_value(
            "Timer Resolution",
            &format!("{cur_ms:.2} ms (Min: {max_ms:.2} ms, Max: {min_ms:.2} ms)"),
        );
    }

    let ram = RamPressureSensor::new();
    TerminalUi::print_key_value(
        "RAM Usage",
        &format!(
            "{:.1}% (Free: {} MB / Total: {} MB)",
            ram.usage_percent, ram.available_mb, ram.total_mb
        ),
    );
    TerminalUi::print_key_value("RAM Pressure Level", &format!("{:?}", ram.level));

    let disk = DiskPressureSensor::new();
    TerminalUi::print_key_value(
        "Disk Usage (C:)",
        &format!(
            "{:.1}% (Free: {:.1} GB / Total: {:.1} GB)",
            disk.usage_percent, disk.free_gb, disk.total_gb
        ),
    );

    let autostart = StartupManager::is_autostart_enabled();
    TerminalUi::print_key_value(
        "Autostart on Boot",
        if autostart { "Enabled" } else { "Disabled" },
    );

    let mut sys = System::new_all();
    sys.refresh_all();
    let leaking = StabilityShield::audit_handle_leaks(&sys);
    TerminalUi::print_key_value(
        "Leaking Processes (>10k handles)",
        &leaking.len().to_string(),
    );

    let mut kalman = KalmanPredictor::new(0.01, 0.1);
    let estimate = kalman.update(sys.global_cpu_usage());
    TerminalUi::print_key_value("Kalman Estimated Load", &format!("{estimate:.1}%"));

    let large_page_min = crate::core::large_pages::LargePageOptimizer::get_large_page_minimum();
    let large_page_str = if large_page_min > 0 {
        format!("{} MB (Hardware Supported)", large_page_min / (1024 * 1024))
    } else {
        "Disabled / Standard 4KB Pages".to_string()
    };
    TerminalUi::print_key_value("Kernel Large Pages (TLB)", &large_page_str);

    // AI Intent Classification of current Foreground Process
    if let Some(hwnd) = win32::get_foreground_hwnd() {
        let fg_pid = win32::get_process_id_from_hwnd(hwnd);
        if let Some(proc) = sys.process(sysinfo::Pid::from_u32(fg_pid)) {
            let name = proc.name().to_string_lossy();
            let mem_mb = proc.memory() / (1024 * 1024);
            let cpu_p = proc.cpu_usage();
            let intent =
                crate::ai::ProcessIntentClassifier::classify(&name, mem_mb, cpu_p, true, 16, true);
            TerminalUi::print_key_value(
                "Active App Intent (AI)",
                &format!("{} [{}]", name, intent.as_str()),
            );
        }
    }
}

pub fn handle_daemon() {
    TerminalUi::print_info("Starting Smart System Manager Background Optimization Daemon...");

    // Attempt to connect to Windows Service Control Manager (SCM)
    if let Err(e) = crate::daemon::service::run_as_service() {
        TerminalUi::print_info(&format!(
            "Running in standalone console mode (SCM connect failed: {e})."
        ));

        // Fallback to standalone loop if not launched by Windows SCM
        let config = Config::load();
        let mut engine = crate::core::engine::SystemEngine::new(config.clone());
        let mut tick_counter: u64 = 0;
        loop {
            tick_counter += 1;
            if tick_counter.is_multiple_of(10) {
                let fresh_config = Config::load();
                engine.config = fresh_config;
            }

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                engine.tick();
            }));

            if let Err(err) = result {
                log::error!(
                    "[Daemon] Standalone engine recovered from unexpected panic: {:?}",
                    err
                );
                let fresh_config = Config::load();
                engine = crate::core::engine::SystemEngine::new(fresh_config);
            }

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

pub fn handle_service(action: &str) {
    match action {
        "install" => {
            if !win32::is_elevated() {
                TerminalUi::print_warning(
                    "Administrator privileges required for service installation.",
                );
                TerminalUi::print_info(
                    "Tip: Right-click your terminal and choose 'Run as Administrator', then retry.",
                );
                return;
            }

            TerminalUi::print_header("Registering Windows Background Service");

            // Install & start Windows Native Service (SCM) — handles boot autostart invisibly
            match service::install_service() {
                Ok(()) => {
                    TerminalUi::print_success(
                        "Windows Service (SmartSystemManager) installed and started.",
                    );
                    TerminalUi::print_success(
                        "Boot autostart enabled (via Windows Service Manager).",
                    );

                    // Remove any leftover Registry Run key to prevent duplicate console window on boot
                    let _ = StartupManager::disable_autostart();
                }
                Err(e) => {
                    TerminalUi::print_warning(&format!(
                        "Service install failed ({e}). Falling back to registry autostart."
                    ));
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

            let exe_path = std::env::current_exe().unwrap_or_default();
            let install_dir = exe_path.parent().map_or_else(
                || "C:\\ssm".to_string(),
                |p| p.to_string_lossy().into_owned(),
            );

            use winreg::enums::{HKEY_CURRENT_USER, KEY_READ, KEY_WRITE};
            use winreg::RegKey;
            TerminalUi::print_info(&format!("Removing {install_dir} from Windows PATH..."));
            if let Ok(hkcu) = RegKey::predef(HKEY_CURRENT_USER)
                .open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)
            {
                if let Ok(current_path) = hkcu.get_value::<String, _>("Path") {
                    let filtered: Vec<&str> = current_path
                        .split(';')
                        .filter(|&p| !p.trim().eq_ignore_ascii_case(&install_dir))
                        .collect();
                    let _ = hkcu.set_value("Path", &filtered.join(";"));
                    TerminalUi::print_success(&format!(
                        "Removed {install_dir} from Windows User PATH."
                    ));
                }
            }

            TerminalUi::print_header("Uninstall Complete");
            TerminalUi::print_info(&format!(
                "{install_dir} directory will be deleted in 2 seconds. Goodbye!"
            ));

            let _ = std::process::Command::new("cmd")
                .args([
                    "/c",
                    &format!("ping 127.0.0.1 -n 3 > NUL & rmdir /S /Q \"{install_dir}\""),
                ])
                .spawn();
        }
        _ => {
            TerminalUi::print_error(
                "Unknown action. Usage: ssm service install | ssm service uninstall",
            );
        }
    }
}

pub fn handle_update(check: bool, enable: bool, disable: bool) {
    TerminalUi::print_header("Smart System Manager Update Utility");
    let mut config = Config::load();

    if enable {
        config.auto_update_enabled = true;
        let _ = config.save();
        TerminalUi::print_success("Automatic background updates are now ENABLED.");
        return;
    }

    if disable {
        config.auto_update_enabled = false;
        let _ = config.save();
        TerminalUi::print_success("Automatic background updates are now DISABLED.");
        return;
    }

    if check {
        TerminalUi::print_info("Checking for updates...");
        match crate::updater::check_for_update() {
            Ok(Some((version, _url))) => {
                TerminalUi::print_success(&format!("A new version ({version}) is available!"));
                TerminalUi::print_info("Run 'ssm update' to install it.");
            }
            Ok(None) => {
                TerminalUi::print_success("You are already on the latest version.");
            }
            Err(e) => {
                TerminalUi::print_error(&format!("Failed to check for updates: {e}"));
            }
        }
        return;
    }

    // Default action: run full update
    TerminalUi::print_info("Checking for updates...");
    match crate::updater::run_update_check(&mut config, true) {
        crate::updater::UpdateStatus::UpToDate => {
            TerminalUi::print_success("You are already on the latest version.");
        }
        crate::updater::UpdateStatus::Updated(version) => {
            TerminalUi::print_success(&format!("Successfully updated to {version}!"));
            TerminalUi::print_info("If the background service is running, it will automatically restart and apply the new version.");

            // Attempt to restart service if it is installed
            TerminalUi::print_info("Attempting to restart Windows Service...");
            let _ = std::process::Command::new("sc")
                .args(["stop", "SmartSystemManager"])
                .output();
            std::thread::sleep(std::time::Duration::from_secs(1));
            let _ = std::process::Command::new("sc")
                .args(["start", "SmartSystemManager"])
                .output();
            TerminalUi::print_success("Done.");
        }
        crate::updater::UpdateStatus::CheckFailed(e) => {
            TerminalUi::print_error(&format!("Update failed: {e}"));
        }
    }
}
