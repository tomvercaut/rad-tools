use clap::Parser;

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
}

#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    todo!("Implement service installation")
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
