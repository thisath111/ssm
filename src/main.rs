#![windows_subsystem = "console"]

pub mod ai;
pub mod cli;
pub mod core;
pub mod daemon;
pub mod sensors;
pub mod updater;
pub mod utils;

use clap::{Parser, Subcommand};
use cli::commands;

#[derive(Parser)]
#[command(
    name = "ssm",
    author = "Smart System Manager Contributors",
    version = env!("CARGO_PKG_VERSION"),
    about = "High-Performance Windows System Optimizer & Ultra-Low Latency Daemon",
    long_about = "A production-grade, open-source Windows performance utility written in native Rust. Optimizes CPU topology, GPU scheduling, RAM standby memory, Win32 timer resolution (0.5ms), and Keyboard/Mouse input latency."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute instant one-click system & hardware performance boost
    Boost,
    /// Purge Windows Standby RAM memory and clean storage temp files
    Clean,
    /// Apply low-latency registry & input response tweaks
    Tune,
    /// Display live hardware topology, timer resolution, and memory stats
    Stats,
    /// Run as a background optimization daemon
    Daemon,
    /// Manage ssm Windows Service (install | uninstall)
    Service {
        /// Action: install | uninstall
        action: String,
    },
    /// Complete uninstallation: revert system tweaks, remove service/autostart/PATH, and delete ssm
    Uninstall,
    /// Update management: check, enable/disable auto-update, or run a manual update
    Update {
        /// Check for updates without installing
        #[arg(long)]
        check: bool,
        /// Enable automatic background updates
        #[arg(long)]
        enable: bool,
        /// Disable automatic background updates
        #[arg(long)]
        disable: bool,
    },
}

fn main() {
    let _ = utils::win32::register_in_path();

    // Setup file and terminal logging
    let exe_dir = std::env::current_exe()
        .unwrap_or_default()
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from("C:\\ssm"));
    let log_path = exe_dir.join("ssm.log");

    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_path)
                .unwrap_or_else(|_| std::fs::File::create("C:\\ssm.log").unwrap()),
        ),
    ])
    .ok();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Boost) => commands::handle_boost(),
        Some(Commands::Clean) => commands::handle_clean(),
        Some(Commands::Tune) => commands::handle_tune(),
        Some(Commands::Stats) => commands::handle_stats(),
        Some(Commands::Daemon) => commands::handle_daemon(),
        Some(Commands::Service { action }) => commands::handle_service(&action),
        Some(Commands::Uninstall) => commands::handle_service("uninstall"),
        Some(Commands::Update {
            check,
            enable,
            disable,
        }) => commands::handle_update(check, enable, disable),
        None => {
            // Default: display live status with banner when no subcommand given
            cli::ui::TerminalUi::print_banner();
            commands::handle_stats();
        }
    }
}
