use clap::Parser;
use rad_tools_dcm_file_sort_service::{run_service, Cli, Config};
use std::sync::Arc;
use tracing::{error, info};

fn main() {
    let cli = Cli::parse();
    let config = Config::try_from(cli);
    if config.is_err() {
        panic!(
            "Unable to create a configuration from commandline arguments: {}",
            config.err().unwrap()
        );
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
    if let Err(e) = run_service(&config, state.clone(), 10) {
        error!("Failed to run service: {:?}", e);
    }
}
