#[cfg(windows)]
use crate::{Cli, Config, ENV_LOG};
#[cfg(windows)]
use clap::Parser;
#[cfg(windows)]
use std::ffi::OsString;
#[cfg(windows)]
use tracing::error;
#[cfg(windows)]
use tracing_subscriber::EnvFilter;

#[cfg(windows)]
pub const NAME: &str = "DicomFileSortService";
#[cfg(windows)]
pub const TYPE: windows_service::service::ServiceType =
    windows_service::service::ServiceType::OWN_PROCESS;
#[cfg(windows)]
pub const DISPLAY_NAME: &str = "Dicom File Sort Service";
#[cfg(windows)]
pub const DESCRIPTION: &str = "Service to sort DICOM files by patient ID and date of birth";

#[cfg(windows)]
pub fn my_service_main(args: Vec<OsString>) {
    let cli = Cli::parse_from(
        args.iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect::<Vec<String>>()
            .as_slice(),
    );
    let config = Config::try_from(cli);
    if config.is_err() {
        error!(
            "Unable to create a configuration from commandline arguments: {}",
            config.err().unwrap()
        );
        return;
    }
    let config = config.unwrap();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env(ENV_LOG))
        .with_thread_ids(true)
        .with_target(true)
        .init();
    if let Err(e) = internal::run_win_service(&config) {
        error!("Error while running the service: {:?}", e);
    }
}

#[cfg(windows)]
mod internal {
    use crate::service::{NAME, TYPE};
    use crate::{Config, ServiceState, run_service};
    use std::time::Duration;
    use tracing::error;
    use windows_service::service::{ServiceControlAccept, ServiceExitCode, ServiceStatus};
    use windows_service::service_control_handler;

    /// Initializes and runs a Windows service with the provided configuration.
    ///
    /// This function:
    /// * Creates a communication channel for service state management
    /// * Registers a Windows service control handler
    /// * Updates service status to Running
    /// * Executes the main service logic
    /// * Updates service status to Stopped when complete
    ///
    /// # Arguments
    ///
    /// * `config` - Reference to the service configuration
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the service runs and stops successfully, or a Windows service error if any operation fails
    pub fn run_win_service(config: &Config) -> windows_service::Result<()> {
        let (tx, rx) = std::sync::mpsc::channel();
        tx.send(ServiceState::Running).unwrap();

        let event_handler = events::handler(tx.clone());

        // Register system service event handler.
        // The returned status handle should be used to report service status changes to the system.
        let status_handle = service_control_handler::register(NAME, event_handler)?;

        // Tell the system that service is running
        status_handle.set_service_status(ServiceStatus {
            service_type: TYPE,
            current_state: windows_service::service::ServiceState::Running,
            controls_accepted: ServiceControlAccept::STOP,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;

        // Start running the internal service
        if let Err(e) = run_service(config, rx) {
            error!("Failed to run service: {:?}", e);
        }

        // Tell the system the service has stopped.
        status_handle.set_service_status(ServiceStatus {
            service_type: TYPE,
            current_state: windows_service::service::ServiceState::Stopped,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;

        Ok(())
    }

    mod events {
        use crate::ServiceState;
        use std::sync::mpsc::Sender;
        use windows_service::service::ServiceControl;
        use windows_service::service_control_handler::ServiceControlHandlerResult;

        /// Creates a Windows service event handler that processes service control events.
        ///
        /// # Arguments
        ///
        /// * `tx` - A channel sender used to communicate service state changes to the main service loop
        ///
        /// # Returns
        ///
        /// Returns a closure that implements the service control event handler. The handler:
        /// * Processes Stop, Shutdown, and Preshutdown events by sending a stop request
        /// * Returns NotImplemented for all other service control events
        /// * Returns NoError on successful processing, or Other(1) if sending the stop request fails
        pub(crate) fn handler(
            tx: Sender<ServiceState>,
        ) -> impl Fn(ServiceControl) -> ServiceControlHandlerResult + Send + 'static {
            move |event: ServiceControl| -> ServiceControlHandlerResult {
                let request_stop = || -> ServiceControlHandlerResult {
                    match tx.send(ServiceState::RequestToStop) {
                        Ok(_) => ServiceControlHandlerResult::NoError,
                        Err(_) => ServiceControlHandlerResult::Other(1),
                    }
                };
                match event {
                    ServiceControl::Preshutdown
                    | ServiceControl::Shutdown
                    | ServiceControl::Stop => request_stop(),
                    _ => ServiceControlHandlerResult::NotImplemented,
                }
            }
        }
    }
}
