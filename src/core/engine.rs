use std::collections::{HashSet, HashMap};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use sysinfo::System;
use crate::utils::{config::Config, win32};
use crate::sensors::{
    audio::AudioSensor, disk_pressure::DiskPressureSensor, gaming::GamingSensor,
};
use crate::core::{
    cpu::CpuManager, dwm::DwmLatencyOptimizer, explorer_watchdog::ExplorerWatchdog,
    freeze_guard::{FreezeGuard, FreezeGuardHeartbeat},
    gpu::GpuManager, input_latency::InputLatencyOptimizer, io_scheduler::IoScheduler,
    large_pages::LargePageOptimizer, math_engine::{KalmanPredictor, PidController},
    network::NetworkOptimizer, nvme_accelerator::NvmeAccelerator, ram::RamManager,
    registry_tweaker::RegistryTweaker, service_tuner::ServiceTuner,
    stability_shield::StabilityShield, timer_resolution::TimerResolutionManager,
};
use crate::ai::{
    ProcessIntent, ProcessIntentClassifier, PredictiveMemoryForecaster,
    SystemWorkloadState, WorkloadStateMachine,
};

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
    boosted_pids: HashMap<u32, u64>,
}

impl SystemEngine {
    pub fn new(config: Config) -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        win32::enable_privilege("SeDebugPrivilege");
        win32::enable_privilege("SeIncreaseBasePriorityPrivilege");
        LargePageOptimizer::enable_large_pages();

        // Initialize watchdog thread
        let heartbeat = FreezeGuardHeartbeat::new();
        let guard_handle = FreezeGuard::spawn(Arc::clone(&heartbeat));

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
        };

        for (pid, _) in engine.sys.processes() {
            engine.active_pids.insert(pid.as_u32());
        }

        let _ = InputLatencyOptimizer::optimize_all();
        let _ = RegistryTweaker::apply_performance_tweaks();
        let _ = DwmLatencyOptimizer::optimize_dwm_latency();
        let _ = NetworkOptimizer::disable_tcp_nagle();
        let _ = NvmeAccelerator::optimize_storage_stack();

        if engine.config.enable_high_precision_timer {
            engine.timer_mgr.enable_high_precision();
        }

        engine
    }

    /// Main optimization loop (1000ms tick).
    pub fn tick(&mut self) {
        self.tick_count += 1;

        // Update heartbeat for watchdog
        self.heartbeat.tick.store(self.tick_count, Ordering::Relaxed);

        let foreground_hwnd = win32::get_foreground_hwnd();
        let foreground_pid = foreground_hwnd
            .map(|h| win32::get_process_id_from_hwnd(h))
            .unwrap_or(0);

        // Tier 1: Fast FG Boost, App Launch Accel & AI Intent
        self.sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

        let mut current_pids = HashSet::new();
        for (pid, process) in self.sys.processes() {
            let p_u32 = pid.as_u32();
            current_pids.insert(p_u32);

            let is_fg = p_u32 == foreground_pid;
            if is_fg {
                let name = process.name().to_string_lossy();
                let mem_mb = process.memory() / (1024 * 1024);
                let cpu_p = process.cpu_usage();
                self.last_detected_intent = ProcessIntentClassifier::classify(
                    &name,
                    mem_mb,
                    cpu_p,
                    true,
                    16,
                );
            }

            // App Launch Acceleration
            if !self.active_pids.contains(&p_u32) {
                let name = process.name().to_string_lossy();
                if p_u32 > 4 && !StabilityShield::is_immune(p_u32, &name) {
                    self.cpu_mgr.boost_process_priority(p_u32);
                    self.boosted_pids.insert(p_u32, self.tick_count);
                }
            }
        }
        self.active_pids = current_pids;

        // Revert expired launch boosts (3s limit)
        let mut expired = Vec::new();
        for (&pid, &start_tick) in self.boosted_pids.iter() {
            if self.tick_count.saturating_sub(start_tick) >= 3 {
                expired.push(pid);
            }
        }
        for pid in expired {
            self.cpu_mgr.restore_process_priority(pid);
            self.boosted_pids.remove(&pid);
        }

        if foreground_pid > 4 && !StabilityShield::is_immune(foreground_pid, "") {
            IoScheduler::prioritize_foreground_process(foreground_pid);
            if self.config.enable_cpu_affinity {
                self.cpu_mgr.pin_foreground(foreground_pid);
            }
        }

        // Pre-emptive Standby Purging
        self.ram_mgr.sensor.update();
        let ram_used_p = self.ram_mgr.sensor.usage_percent;
        self.ai_forecaster.record_and_predict(ram_used_p);

        if self.config.enable_standby_purging && self.ai_forecaster.should_preemptively_purge(ram_used_p) {
            self.ram_mgr.purge_standby_memory();
        }

        // Tier 2: AI State Machine (3s interval)
        if self.tick_count % 3 == 0 {
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
                ServiceTuner::pause_background_services();
            } else {
                self.cpu_mgr.restore_default_mode();
                self.gpu_mgr.restore_default();
                ServiceTuner::restore_background_services();
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
                if p_u32 == foreground_pid || p_u32 == self_pid || StabilityShield::is_immune(p_u32, &name) {
                    continue;
                }

                // Aggressive Network Burst Throttling
                let is_notorious = name == "compattelrunner.exe" || name == "tiworker.exe" 
                                || name == "wermgr.exe" || name == "mousocoreworker.exe" 
                                || name == "mrt.exe" || name == "backgroundtaskhost.exe";

                if is_notorious {
                    IoScheduler::deprioritize_background_process(p_u32);
                    self.cpu_mgr.throttle_process_priority(p_u32);
                    if self.config.enable_cpu_affinity {
                        self.cpu_mgr.pin_background(p_u32);
                    }
                }
            }
        }

        // Tier 3: Audio & WorkingSet Maintenance (10s interval)
        if self.tick_count % 10 == 0 {
            self.cached_audio_pids = self.audio_sensor.get_active_audio_pids();

            let mut protected = vec![foreground_pid, std::process::id()];
            protected.extend(self.cached_audio_pids.iter());

            self.ram_mgr.trim_background_working_sets(
                &self.sys,
                &protected,
                self.tick_count,
            );
            self.explorer_guard.check(
                &self.sys,
                &self.ram_mgr,
                self.config.explorer_memory_limit_mb,
                self.tick_count,
            );
        }

        // Tier 4: Storage & Standby Purge (30s interval)
        if self.tick_count % 30 == 0 {
            self.disk_sensor.update();
            if self.disk_sensor.usage_percent > self.config.disk_auto_clean_percent {
                self.disk_sensor.clean_temp_files();
            }

            if self.config.enable_standby_purging && self.ram_mgr.sensor.available_mb < 2000 {
                self.ram_mgr.purge_standby_memory();
            }
        }
    }
}
