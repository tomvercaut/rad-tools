use clap::Parser;
use rad_tools_dcm_file_sort_service::{Cli, Config, run_service};
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
        .with_file(true)
        .with_line_number(true)
        .with_max_level(config.log.level)
        .init();

    let (tx, rx) = std::sync::mpsc::channel();
    {
        let tx = tx.clone();
        ctrlc::set_handler(move || {
            tx.send(rad_tools_dcm_file_sort_service::ServiceState::RequestToStop)
                .expect("Failed to send request to stop signal");
        })
        .expect("Error setting Ctrl-C handler");
    }
    info!("Waiting for Ctrl-C ...");
    if let Err(e) = run_service(&config, rx) {
        error!("Failed to run service: {:?}", e);
    }
}
