use clap::Parser;
use rad_tools_dcm_file_sort::{Cli, Config, ServiceState, run_service};
use tracing::{error, info};

fn main() {
    rad_tools_core::tracing::default_env_subscriber().init();
    let cli = Cli::parse();
    let config = Config::try_from(cli);
    if config.is_err() {
        panic!(
            "Unable to create a configuration from commandline arguments: {}",
            config.err().unwrap()
        );
    }
    let config = config.unwrap();

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
