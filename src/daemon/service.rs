use crate::core::engine::SystemEngine;
use crate::utils::config::Config;
use std::ffi::OsString;
use std::os::windows::process::CommandExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher,
};

pub const SERVICE_NAME: &str = "SmartSystemManager";
static SHOULD_EXIT: AtomicBool = AtomicBool::new(false);

define_windows_service!(ffi_service_main, my_service_main);

pub fn run_as_service() -> Result<(), Box<dyn std::error::Error>> {
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)?;
    Ok(())
}

fn my_service_main(_arguments: Vec<OsString>) {
    let _ = service_execution_loop();
}

fn service_execution_loop() -> Result<(), windows_service::Error> {
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop => {
                SHOULD_EXIT.store(true, Ordering::Release);
                ServiceControlHandlerResult::NoError
            }
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    let config = Config::load();
    let mut engine = SystemEngine::new(config);

    loop {
        if SHOULD_EXIT.load(Ordering::Acquire) {
            break;
        }

        engine.tick();
        thread::sleep(Duration::from_secs(1));
    }

    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    Ok(())
}

pub fn install_service() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = std::env::current_exe()?;
    let exe_str = exe_path
        .to_str()
        .ok_or("Executable path contains non-UTF8 characters")?;

    // Stop existing instance
    let _ = std::process::Command::new("sc")
        .args(["stop", SERVICE_NAME])
        .creation_flags(0x0800_0000)
        .output();

    let query = std::process::Command::new("sc")
        .args(["query", SERVICE_NAME])
        .creation_flags(0x0800_0000)
        .output()?;

    // Check if service exists
    let service_exists = String::from_utf8_lossy(&query.stdout).contains("STATE");

    if service_exists {
        std::process::Command::new("sc")
            .args([
                "config",
                SERVICE_NAME,
                &format!("binPath=\"{exe_str}\" daemon"),
                "start=auto",
            ])
            .creation_flags(0x0800_0000)
            .output()?;
    } else {
        std::process::Command::new("sc")
            .args([
                "create",
                SERVICE_NAME,
                &format!("binPath=\"{exe_str}\" daemon"),
                "start=auto",
                "obj=LocalSystem",
            ])
            .creation_flags(0x0800_0000)
            .output()?;
    }

    let _ = std::process::Command::new("sc")
        .args(["start", SERVICE_NAME])
        .creation_flags(0x0800_0000)
        .output();

    // Configure SCM auto-restart (3 attempts, 1s delay)
    let _ = std::process::Command::new("sc")
        .args([
            "failure",
            SERVICE_NAME,
            "reset=60",
            "actions=restart/1000/restart/1000/restart/1000",
        ])
        .creation_flags(0x0800_0000)
        .output();

    Ok(())
}

pub fn uninstall_service() -> Result<(), Box<dyn std::error::Error>> {
    let _ = std::process::Command::new("sc")
        .args(["stop", SERVICE_NAME])
        .creation_flags(0x0800_0000)
        .output();

    std::process::Command::new("sc")
        .args(["delete", SERVICE_NAME])
        .creation_flags(0x0800_0000)
        .output()?;

    Ok(())
}
