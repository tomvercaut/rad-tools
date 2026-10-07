use clap::Parser;
use rad_tools_service_manager::uninstall_local_service;
use tracing::error;

/// An application to uninstall a Windows service based on a configuration file.
///
/// The application uninstalls a Windows service using a TOML configuration file.
#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "
An application to uninstall a Windows service based on a configuration file.

The application uninstalls a Windows service using a TOML configuration file."
)]
struct Cli {
    /// Configuration file in TOML format in which the service parameters are defined.
    config: String,
}

#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    rad_tools_core::tracing::default_env_subscriber().init();
    let cli = Cli::parse();

    let path = std::path::Path::new(&cli.config);

    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read config file: {}", e));
    let config: rad_tools_service_manager::ServiceConfig =
        toml::from_str(&content).unwrap_or_else(|e| panic!("Failed to parse TOML config: {}", e));

    if let Err(e) = uninstall_local_service(&config) {
        error!("Failed to uninstall service: {}", e);
    }

    Ok(())
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
