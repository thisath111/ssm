#![windows_subsystem = "console"]

pub mod cli;
pub mod core;
pub mod daemon;
pub mod sensors;
pub mod utils;

use clap::{Parser, Subcommand};
use cli::commands;

#[derive(Parser)]
#[command(
    name = "ssm",
    author = "Smart System Manager Contributors",
    version = "1.0.0",
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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = utils::win32::register_in_path();

    simplelog::TermLogger::init(
        simplelog::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    ).ok();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Boost) => commands::handle_boost(),
        Some(Commands::Clean) => commands::handle_clean(),
        Some(Commands::Tune) => commands::handle_tune(),
        Some(Commands::Stats) => commands::handle_stats(),
        Some(Commands::Daemon) => commands::handle_daemon(),
        Some(Commands::Service { action }) => commands::handle_service(&action),
        Some(Commands::Uninstall) => commands::handle_service("uninstall"),
        None => {
            // Default: display live status when no subcommand given
            commands::handle_stats();
        }
    }

    Ok(())
}
