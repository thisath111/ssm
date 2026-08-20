use std::collections::HashSet;
use sysinfo::System;
use crate::utils::{config::Config, win32};
use crate::sensors::{
    audio::AudioSensor, disk_pressure::DiskPressureSensor, gaming::GamingSensor,
};
use crate::core::{
    cpu::CpuManager, explorer_watchdog::ExplorerWatchdog, gpu::GpuManager,
    input_latency::InputLatencyOptimizer, io_scheduler::IoScheduler,
    math_engine::{KalmanPredictor, PidController}, network::NetworkOptimizer,
    nvme_accelerator::NvmeAccelerator, ram::RamManager, registry_tweaker::RegistryTweaker,
    service_tuner::ServiceTuner, stability_shield::StabilityShield,
    timer_resolution::TimerResolutionManager,
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
    tick_count: u64,
    cached_audio_pids: HashSet<u32>,
}

impl SystemEngine {
    pub fn new(config: Config) -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        win32::enable_privilege("SeDebugPrivilege");
        win32::enable_privilege("SeIncreaseBasePriorityPrivilege");

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
            tick_count: 0,
            cached_audio_pids: HashSet::new(),
        };

        let _ = InputLatencyOptimizer::optimize_all();
        let _ = RegistryTweaker::apply_performance_tweaks();
        let _ = NetworkOptimizer::disable_tcp_nagle();
        let _ = NvmeAccelerator::optimize_storage_stack();

        if engine.config.enable_high_precision_timer {
            engine.timer_mgr.enable_high_precision();
        }

        engine
    }

    /// Single optimization tick — executed by daemon/service loop every 1000ms.
    pub fn tick(&mut self) {
        self.tick_count += 1;

        let foreground_hwnd = win32::get_foreground_hwnd();
        let foreground_pid = foreground_hwnd
            .map(|h| win32::get_process_id_from_hwnd(h))
            .unwrap_or(0);

        // Tier 1: Fast Foreground Boost & High-Precision Timer Check (Every 1s tick)
        if foreground_pid > 4 && !StabilityShield::is_immune(foreground_pid, "") {
            IoScheduler::prioritize_foreground_process(foreground_pid);
            if self.config.enable_cpu_affinity {
                self.cpu_mgr.pin_foreground(foreground_pid);
            }
        }

        // Tier 2: Process Scan, Kalman Predictor & PID Balancing (Every 3s)
        if self.tick_count % 3 == 0 {
            self.sys.refresh_cpu_usage();
            self.sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

            let raw_cpu = self.sys.global_cpu_usage();
            let estimated_cpu = self.kalman_cpu.update(raw_cpu);
            let pid_output = self.pid_controller.compute(80.0, estimated_cpu, 3.0);

            let is_gaming = self.gaming_sensor.is_gaming_active(&self.sys) || estimated_cpu > 75.0 || pid_output < 0.0;
            if is_gaming {
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

            self.ram_mgr.sensor.update();
            let self_pid = std::process::id();

            // Audit handle leaks & deprioritize background hogs
            let leaking = StabilityShield::audit_handle_leaks(&self.sys);
            for (pid, _, _) in leaking {
                IoScheduler::deprioritize_background_process(pid);
            }

            for (pid, process) in self.sys.processes() {
                let p_u32 = pid.as_u32();
                if p_u32 == foreground_pid || p_u32 == self_pid || StabilityShield::is_immune(p_u32, &process.name().to_string_lossy()) {
                    continue;
                }

                if process.cpu_usage() > 15.0 {
                    IoScheduler::deprioritize_background_process(p_u32);
                    if self.config.enable_cpu_affinity {
                        self.cpu_mgr.pin_background(p_u32);
                    }
                }
            }
        }

        // Tier 3: Audio & WorkingSet Maintenance (Every 10s)
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

        // Tier 4: Storage NVMe & Standby Memory Purge (Every 30s)
        if self.tick_count % 30 == 0 {
            self.disk_sensor.update();
            if self.disk_sensor.usage_percent > self.config.disk_auto_clean_percent {
                self.disk_sensor.clean_temp_files();
            }

            if self.config.enable_standby_purging
                && self.ram_mgr.sensor.available_mb < 2000
            {
                self.ram_mgr.purge_standby_memory();
            }
        }
    }
}
