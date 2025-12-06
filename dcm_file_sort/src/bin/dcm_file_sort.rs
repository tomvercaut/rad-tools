use clap::Parser;
use rad_tools_dcm_file_sort::{Cli, Config, ENV_LOG, run_service, ServiceState};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

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
        .with_env_filter(EnvFilter::from_env(ENV_LOG))
        .with_thread_ids(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    let (tx, rx) = std::sync::mpsc::channel();
    {
        let tx = tx.clone();
        ctrlc::set_handler(move || {
            tx.send(ServiceState::RequestToStop)
                .expect("Failed to send a request to stop signal");
        })
        .expect("Error setting Ctrl-C handler");
    }
    info!("Waiting for Ctrl-C ...");
    if let Err(e) = run_service(&config, rx) {
        error!("Failed to run service: {:?}", e);
    }
}
