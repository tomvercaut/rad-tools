use clap::Parser;
use rad_tools_dcm_file_sort_service::{run_service, Cli};
use std::sync::Arc;
use tracing::{error, info, Level};

fn main() {
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
        .with_max_level(level)
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
    if let Err(e) = run_service(cli.input_dir, cli.output_dir, state.clone(), 10) {
        error!("Failed to run service: {:?}", e);
    }
}
