use clap::Parser;
use rad_tools_service_manager::create_local_service;
use tracing::{Level, error};

/// An application to install a Windows service based on a configuration file.
///
/// The application installs a Windows service using a TOML configuration file.
#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "
An application to install a Windows service based on a configuration file.

The application installs a Windows service using a TOML configuration file."
)]
struct Cli {
    /// Configuration file in TOML format in which the service parameters are defined.
    config: String,
    /// Enable logging at INFO level.
    #[arg(long, default_value_t = false)]
    pub verbose: bool,
    /// Enable logging at DEBUG level.
    #[arg(long, default_value_t = false)]
    pub debug: bool,
    /// Enable logging at TRACE level.
    #[arg(long, default_value_t = false)]
    pub trace: bool,
}

#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    let cli = Cli::parse();
    let level = if cli.trace {
        Level::TRACE
    } else if cli.debug {
        Level::DEBUG
    } else if cli.verbose {
        Level::INFO
    } else {
        Level::WARN
    };
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_max_level(level)
        .init();

    let path = std::path::Path::new(&cli.config);

    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read config file: {}", e));
    let config: rad_tools_service_manager::ServiceConfig =
        toml::from_str(&content).unwrap_or_else(|e| panic!("Failed to parse TOML config: {}", e));

    if let Err(e) = create_local_service(&config) {
        error!("Failed to create service: {}", e);
    }

    Ok(())
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
