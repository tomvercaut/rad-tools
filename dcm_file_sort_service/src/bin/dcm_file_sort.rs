use clap::Parser;
use rad_tools_dcm_file_sort_service::{run_service, Cli, Config};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{error, info, Level};

fn main() {
    let cli = Cli::parse();
    let config = if let Some(args) = cli.manual_args {
        let mut config = Config::default();
        config.paths.input_dir = PathBuf::from(args.input_dir);
        config.paths.output_dir = PathBuf::from(args.output_dir);
        config.paths.unknown_dir = PathBuf::from(args.unknown_dir);
        config.log.level = if args.trace {
            Level::TRACE
        } else if args.debug {
            Level::DEBUG
        } else if args.verbose {
            Level::INFO
        } else {
            Level::WARN
        };
        Some(config)
    } else if let Some(config_path) = cli.config {
        let config_content =
            std::fs::read_to_string(config_path).expect("Failed to read the config file");
        let config: Config =
            toml::from_str(&config_content).expect("Failed to parse the config file");
        Some(config)
    } else {
        None
    };
    if config.is_none() {
        panic!("No config arguments or file provided");
    }
    let config = config.unwrap();
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_target(true)
        .with_max_level(config.log.level)
        .init();

    let state = Arc::new(std::sync::RwLock::new(
        rad_tools_dcm_file_sort_service::ServiceState::Running,
    ));
    {
        let moved_state = state.clone();
        ctrlc::set_handler(move || match moved_state.try_write() {
            Ok(mut inner) => {
                *inner = rad_tools_dcm_file_sort_service::ServiceState::RequestToStop;
            }
            Err(_) => {
                error!("Failed to get write lock on service state.");
            }
        })
        .expect("Error setting Ctrl-C handler");
    }
    info!("Waiting for Ctrl-C ...");
    if let Err(e) = run_service(
        config.paths.input_dir,
        config.paths.output_dir,
        state.clone(),
        10,
    ) {
        error!("Failed to run service: {:?}", e);
    }
}
