use simplelog::*;

pub fn init_logger() {
    let _ = TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );

    log::info!("Logger initialized.");
}
