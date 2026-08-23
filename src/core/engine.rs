use crate::ai::{
    HardwareProfile, PredictiveMemoryForecaster, ProcessIntent, ProcessIntentClassifier,
    SystemWorkloadState, WorkloadStateMachine,
};
use crate::core::{
    cpu::CpuManager,
    dwm::DwmLatencyOptimizer,
    explorer_watchdog::ExplorerWatchdog,
    freeze_guard::{FreezeGuard, FreezeGuardHeartbeat},
    gpu::GpuManager,
    input_latency::InputLatencyOptimizer,
    io_scheduler::IoScheduler,
    large_pages::LargePageOptimizer,
    math_engine::{KalmanPredictor, PidController},
    network::NetworkOptimizer,
    nvme_accelerator::NvmeAccelerator,
    ram::RamManager,
    registry_tweaker::RegistryTweaker,
    service_tuner::ServiceTuner,
    stability_shield::StabilityShield,
    timer_resolution::TimerResolutionManager,
};
use crate::sensors::{audio::AudioSensor, disk_pressure::DiskPressureSensor, gaming::GamingSensor};
use crate::utils::{config::Config, win32};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use sysinfo::System;

pub struct SystemEngine {
    pub sys: System,
    pub config: Config,
    pub timer_mgr: TimerResolutionManager,
    pub cpu_mgr: CpuManager,
    pub gpu_mgr: GpuManager,
    pub ram_mgr: RamManager,
    pub disk_sensor: DiskPressureSensor,
    pub gaming_sensor: GamingSensor,
    pub audio_sensor: AudioSensor,
    pub explorer_guard: ExplorerWatchdog,
    pub pid_controller: PidController,
    pub kalman_cpu: KalmanPredictor,
    pub ai_forecaster: PredictiveMemoryForecaster,
    pub ai_workload_state: WorkloadStateMachine,
    pub last_detected_intent: ProcessIntent,
    heartbeat: Arc<FreezeGuardHeartbeat>,
    _freeze_guard_handle: Option<std::thread::JoinHandle<()>>,
    tick_count: u64,
    cached_audio_pids: HashSet<u32>,
    active_pids: HashSet<u32>,
    boosted_pids: HashMap<u32, (u64, u32)>,
    last_foreground_pid: u32,
    pub service_tuner: ServiceTuner,
    pub hardware_profile: HardwareProfile,

    // Zero-Allocation Buffers
    current_pids_buf: HashSet<u32>,
    expired_boosts_buf: Vec<(u32, u32)>,
}

impl SystemEngine {
    #[must_use] 
    pub fn new(config: Config) -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        win32::enable_privilege("SeDebugPrivilege");
        win32::enable_privilege("SeIncreaseBasePriorityPrivilege");
        LargePageOptimizer::enable_large_pages();

        // Initialize watchdog thread
        let heartbeat = FreezeGuardHeartbeat::new();
        let guard_handle = FreezeGuard::spawn(Arc::clone(&heartbeat));

        let hardware_profile = HardwareProfile::auto_detect(&sys);

        let mut engine = Self {
            sys,
            config,
            timer_mgr: TimerResolutionManager::new(),
            cpu_mgr: CpuManager::new(),
            gpu_mgr: GpuManager::new(),
            ram_mgr: RamManager::new(),
            disk_sensor: DiskPressureSensor::new(),
            gaming_sensor: GamingSensor::new(),
            audio_sensor: AudioSensor::new(),
            explorer_guard: ExplorerWatchdog::new(),
            pid_controller: PidController::new(1.2, 0.1, 0.05, 0.0, 100.0),
            kalman_cpu: KalmanPredictor::new(0.01, 0.1),
            ai_forecaster: PredictiveMemoryForecaster::new(),
            ai_workload_state: WorkloadStateMachine::new(),
            last_detected_intent: ProcessIntent::InteractiveUi,
            heartbeat,
            _freeze_guard_handle: Some(guard_handle),
            tick_count: 0,
            cached_audio_pids: HashSet::new(),
            active_pids: HashSet::new(),
            boosted_pids: HashMap::new(),
            last_foreground_pid: 0,
            service_tuner: ServiceTuner::new(),
            hardware_profile,
            current_pids_buf: HashSet::with_capacity(512),
            expired_boosts_buf: Vec::with_capacity(32),
        };

        for pid in engine.sys.processes().keys() {
            engine.active_pids.insert(pid.as_u32());
        }

        if let Err(e) = InputLatencyOptimizer::optimize_all() {
            log::warn!("Failed to optimize input latency: {e}");
        }
        if let Err(e) = RegistryTweaker::apply_performance_tweaks() {
            log::warn!("Failed to apply registry tweaks: {e}");
        }
        if let Err(e) = DwmLatencyOptimizer::optimize_dwm_latency() {
            log::warn!("Failed to optimize DWM latency: {e}");
        }
        if let Err(e) = NetworkOptimizer::disable_tcp_nagle() {
            log::warn!("Failed to optimize network: {e}");
        }
        if let Err(e) = NvmeAccelerator::optimize_storage_stack() {
            log::warn!("Failed to optimize NVMe stack: {e}");
        }

        // Apply hardware-adaptive timer resolution to avoid DPC overhead on low-end PCs
        if engine.config.enable_high_precision_timer {
            let resolution = engine.hardware_profile.optimal_timer_resolution_100ns();
            engine.timer_mgr.enable_adaptive(resolution);
        }

        // Wire hardware-adaptive emergency thresholds into AI State Machine
        engine.ai_workload_state.emergency_ram_threshold =
            engine.hardware_profile.emergency_ram_threshold();
        engine.ai_workload_state.emergency_cpu_threshold = match engine.hardware_profile.tier {
            crate::ai::HardwareTier::LowEndBudget => 88.0,
            crate::ai::HardwareTier::MidRangeStandard => 92.0,
            crate::ai::HardwareTier::HighEndEnthusiast => 95.0,
        };

        engine
    }

    /// Main optimization loop (1000ms tick).
    pub fn tick(&mut self) {
        self.tick_count += 1;

        // Update heartbeat for watchdog
        self.heartbeat
            .tick
            .store(self.tick_count, Ordering::Relaxed);

        let foreground_hwnd = win32::get_foreground_hwnd();
        let foreground_pid = foreground_hwnd
            .map_or(0, win32::get_process_id_from_hwnd);

        // Tier 1: Fast FG Boost, App Launch Accel & AI Intent
        self.sys
            .refresh_processes(sysinfo::ProcessesToUpdate::All, true);

        let window_pids = win32::get_all_window_owner_pids();

        self.current_pids_buf.clear();
        for (pid, process) in self.sys.processes() {
            let p_u32 = pid.as_u32();
            self.current_pids_buf.insert(p_u32);

            let is_fg = p_u32 == foreground_pid;
            let has_win = window_pids.contains(&p_u32);
            if is_fg {
                let name = process.name().to_string_lossy();
                let mem_mb = process.memory() / (1024 * 1024);
                let cpu_p = process.cpu_usage();
                self.last_detected_intent =
                    ProcessIntentClassifier::classify(&name, mem_mb, cpu_p, true, 16, has_win);
            }

            // App Launch Acceleration
            if !self.active_pids.contains(&p_u32) {
                let name = process.name().to_string_lossy();
                if p_u32 > 4 && !StabilityShield::is_immune(p_u32, &name) {
                    if let Some(orig_prio) = self.cpu_mgr.boost_process_priority(p_u32) {
                        self.boosted_pids
                            .insert(p_u32, (self.tick_count, orig_prio));
                    }
                }
            }
        }
        std::mem::swap(&mut self.active_pids, &mut self.current_pids_buf);

        // Revert expired launch boosts (3s limit)
        self.expired_boosts_buf.clear();
        for (&pid, &(start_tick, orig_prio)) in &self.boosted_pids {
            if self.tick_count.saturating_sub(start_tick) >= 3 {
                self.expired_boosts_buf.push((pid, orig_prio));
            }
        }
        for (pid, orig_prio) in &self.expired_boosts_buf {
            self.cpu_mgr.restore_process_priority(*pid, *orig_prio);
            self.boosted_pids.remove(pid);
        }

        if foreground_pid != self.last_foreground_pid {
            // Restore I/O priority for the previous foreground process
            if self.last_foreground_pid > 4 {
                IoScheduler::restore_process_io(self.last_foreground_pid);
            }

            // Prioritize the new foreground process
            if foreground_pid > 4 {
                let foreground_name = self
                    .sys
                    .process(sysinfo::Pid::from_u32(foreground_pid))
                    .map(|p| p.name().to_string_lossy().to_string())
                    .unwrap_or_default();
                if !StabilityShield::is_immune(foreground_pid, &foreground_name) {
                    IoScheduler::prioritize_foreground_process(foreground_pid);
                    if self.config.enable_cpu_affinity {
                        self.cpu_mgr.pin_foreground(foreground_pid);
                    }
                }
            }
            self.last_foreground_pid = foreground_pid;
        }

        // Pre-emptive Standby Purging
        self.ram_mgr.sensor.update();
        let ram_used_p = self.ram_mgr.sensor.usage_percent;
        self.ai_forecaster.record_and_predict(ram_used_p);

        if self.config.enable_standby_purging
            && self.ai_forecaster.should_preemptively_purge(ram_used_p)
        {
            self.ram_mgr.purge_standby_memory();
        }

        // Tier 2: AI State Machine (3s interval)
        if self.tick_count.is_multiple_of(3) {
            self.sys.refresh_cpu_usage();

            let raw_cpu = self.sys.global_cpu_usage();
            let estimated_cpu = self.kalman_cpu.update(raw_cpu);
            let is_audio_active = !self.cached_audio_pids.is_empty();

            let state = self.ai_workload_state.evaluate(
                estimated_cpu,
                ram_used_p,
                self.last_detected_intent,
                is_audio_active,
            );

            let is_boost_needed = state == SystemWorkloadState::UltraGaming
                || state == SystemWorkloadState::CreatorDeveloperBoost
                || estimated_cpu > 75.0;

            if is_boost_needed {
                if self.config.enable_power_plan_boost {
                    self.cpu_mgr.enable_performance_mode();
                }
                if self.config.enable_gpu_boost {
                    self.gpu_mgr.enable_gpu_boost();
                }
                let _ = NetworkOptimizer::enable_qos_policy();
                self.service_tuner.pause_background_services();
            } else {
                self.cpu_mgr.restore_default_mode();
                self.gpu_mgr.restore_default();
                self.service_tuner.restore_background_services();
            }

            let self_pid = std::process::id();

            // Audit handle leaks & throttle hogs
            let leaking = StabilityShield::audit_handle_leaks(&self.sys);
            for (pid, _, _) in leaking {
                IoScheduler::deprioritize_background_process(pid);
            }

            for (pid, process) in self.sys.processes() {
                let p_u32 = pid.as_u32();
                let name = process.name().to_string_lossy().to_lowercase();
                // Dynamic check: NEVER throttle active interactive/typing window owners!
                if p_u32 == foreground_pid
                    || p_u32 == self_pid
                    || StabilityShield::is_immune(p_u32, &name)
                    || window_pids.contains(&p_u32)
                {
                    continue;
                }

                // Behavioral detection: High I/O + Low CPU = background disk hog (no name hardcoding)
                let mem_mb = process.memory() / (1024 * 1024);
                let cpu_p = process.cpu_usage();
                let is_bg_disk_hog = cpu_p < 5.0
                    && mem_mb < self.hardware_profile.ram_pressure_trim_mb * 2
                    && process.status() != sysinfo::ProcessStatus::Run;

                if is_bg_disk_hog {
                    if self.hardware_profile.enable_aggressive_io_throttle {
                        unsafe {
                            if let Ok(handle) = windows::Win32::System::Threading::OpenProcess(
                                windows::Win32::System::Threading::PROCESS_SET_INFORMATION,
                                false,
                                p_u32,
                            ) {
                                crate::utils::nt_api::set_process_io_priority(
                                    handle,
                                    self.hardware_profile.max_background_io_prio,
                                );
                                let _ = windows::Win32::Foundation::CloseHandle(handle);
                            }
                        }
                    } else {
                        IoScheduler::deprioritize_background_process(p_u32);
                    }
                    self.cpu_mgr.throttle_process_priority(p_u32);
                    if self.config.enable_cpu_affinity {
                        self.cpu_mgr.pin_background(p_u32);
                    }
                }
            }
        }

        // Tier 3: Audio & WorkingSet Maintenance (10s interval)
        if self.tick_count.is_multiple_of(10) {
            self.cached_audio_pids = self.audio_sensor.get_active_audio_pids();

            let mut protected = vec![foreground_pid, std::process::id()];
            protected.extend(self.cached_audio_pids.iter());
            protected.extend(window_pids.iter()); // Protect all interactive/typing window owners dynamically!

            self.ram_mgr.trim_background_working_sets(
                &self.sys,
                &protected,
                self.tick_count,
                self.hardware_profile.ram_pressure_trim_mb,
            );
            self.explorer_guard.check(
                &self.sys,
                &self.ram_mgr,
                self.config.explorer_memory_limit_mb,
                self.tick_count,
            );
        }

        // Tier 4: Storage & Standby Purge (30s interval)
        if self.tick_count.is_multiple_of(30) {
            self.disk_sensor.update();
            if self.disk_sensor.usage_percent > self.config.disk_auto_clean_percent {
                self.disk_sensor.clean_temp_files();
            }

            // Standby purge threshold: hardware-adaptive (Low-end PCs need more aggressive purging)
            let standby_purge_threshold_mb = match self.hardware_profile.tier {
                crate::ai::HardwareTier::LowEndBudget => self.hardware_profile.total_ram_mb / 8, // 12.5% of total RAM
                crate::ai::HardwareTier::MidRangeStandard => self.hardware_profile.total_ram_mb / 6, // ~16%
                crate::ai::HardwareTier::HighEndEnthusiast => {
                    self.hardware_profile.total_ram_mb / 4
                } // 25%
            };
            if self.config.enable_standby_purging
                && self.ram_mgr.sensor.available_mb < standby_purge_threshold_mb
            {
                self.ram_mgr.purge_standby_memory();
            }
        }
    }
}
