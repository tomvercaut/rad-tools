use anyhow::anyhow;
use clap::{Parser, Subcommand};
use rad_tools_common::Validate;
use rad_tools_dcm_forward::config::{
    Config, DicomListener, DicomStreamEndpoint, DirEndpoint, Endpoint, Manager, Route,
};
use std::path::PathBuf;
use tracing::error;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Args {
    #[command(subcommand)]
    commands: Option<Commands>,

    /// Enable logging at INFO level.
    #[arg(long, default_value_t = false)]
    verbose: bool,
    /// Enable logging at DEBUG level.
    #[arg(long, default_value_t = false)]
    debug: bool,
    /// Enable logging at TRACE level.
    #[arg(long, default_value_t = false)]
    trace: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a sample configuration file.
    Generate,
    /// Show the current configuration.
    Show { config: Option<String> },
    /// Start the application
    Start { config: Option<String> },
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let level = rad_tools_common::get_log_level!(args);
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_target(true)
        .with_max_level(level)
        .init();

    if let Some(commands) = args.commands {
        match commands {
            Commands::Generate => {
                let sample_config = sample_config();
                let toml =
                    toml::to_string_pretty(&sample_config).expect("Failed to create TOML config.");
                println!("{}", toml);
                Ok(())
            }
            Commands::Show { config } => {
                let config = get_config(config)?;
                println!("{:#?}", toml::to_string_pretty(&config));
                Ok(())
            }
            Commands::Start { config } => {
                let config = get_config(config)?;
                // Start the configuration validation
                config.validate()?;
                // Start the listeners
                Ok(())
            }
        }
    } else {
        Err(anyhow::anyhow!("No supported command is specified."))
    }
}

/// Determines the configuration file path based on the provided input or falls back to a default location.
///
/// # Arguments
/// * `config` - Optional string containing the path to the configuration file
///
/// # Returns
/// * `Ok(PathBuf)` - Path to the configuration file
/// * `Err` - If unable to determine the configuration file path
///
/// If a configuration path is provided, it is used directly. Otherwise, the function looks for
/// 'config.toml' in the same directory as the executable.
fn config_path_or_default(config: Option<String>) -> Result<PathBuf, anyhow::Error> {
    if let Some(config) = config {
        Ok(PathBuf::from(config))
    } else {
        let exe = std::env::current_exe().map_err(|e| {
            error!("{:#?}", e);
            anyhow::anyhow!("Failed to get the current executable path.")
        })?;
        exe.parent()
            .ok_or_else(|| anyhow::anyhow!("Failed to get the parent directory of the executable."))
            .map(|p| p.join("config.toml").to_path_buf())
    }
}

/// Loads and parses the configuration file.
///
/// # Arguments
/// * `config` - Optional string containing the path to the configuration file
///
/// # Returns
/// * `Ok(Config)` - Parsed configuration structure
/// * `Err` - If unable to read or parse the configuration file
///
/// This function determines the configuration file location, reads its contents,
/// and parses it as a TOML file into the Config structure.
fn get_config(config: Option<String>) -> Result<Config, anyhow::Error> {
    let config = config_path_or_default(config)?;
    let content = std::fs::read_to_string(&config).map_err(|e| {
        error!("{:#?}", e);
        anyhow!("Failed to read config.")
    })?;
    let parsed_config = toml::from_str(&content).map_err(|e| {
        error!("{:#?}", e);
        anyhow!("Failed to parse config.")
    })?;

    Ok(parsed_config)
}

fn sample_config() -> Config {
    let temp_dir = std::env::temp_dir();
    let listeners = vec![
        DicomListener {
            name: "Listener_1".to_string(),
            port: 104,
            ae: "AE_1".to_string(),
            output: temp_dir.join("Listener_1").to_str().unwrap().to_string(),
        },
        DicomListener {
            name: "Listener_2".to_string(),
            port: 105,
            ae: "AE_2".to_string(),
            output: temp_dir.join("Listener_2").to_str().unwrap().to_string(),
        },
    ];
    let endpoints = vec![
        Endpoint::Dicom(DicomStreamEndpoint {
            name: "DSE_1".to_string(),
            addr: "192.168.1.10".to_string(),
            port: 106,
            aet: "AET_1".to_string(),
            aec: "DSE_1".to_string(),
        }),
        Endpoint::Dicom(DicomStreamEndpoint {
            name: "DSE_2".to_string(),
            addr: "192.168.2.10".to_string(),
            port: 107,
            aet: "AET_2".to_string(),
            aec: "DSE_2".to_string(),
        }),
        Endpoint::Dir(DirEndpoint {
            name: "DE_1".to_string(),
            path: temp_dir.join("DE_1").to_str().unwrap().to_string(),
        }),
        Endpoint::Dir(DirEndpoint {
            name: "DE_2".to_string(),
            path: temp_dir.join("DE_2").to_str().unwrap().to_string(),
        }),
    ];
    let routes = vec![
        Route {
            name: "Listener_1".to_string(),
            endpoints: vec!["DSE_1".to_string(), "DE_1".to_string()],
        },
        Route {
            name: "Listener_2".to_string(),
            endpoints: vec!["DSE_2".to_string(), "DE_2".to_string()],
        },
    ];
    Config {
        listeners,
        endpoints,
        routes,
        manager: Manager {
            max_stop_attempts: 100,
        },
    }
}
