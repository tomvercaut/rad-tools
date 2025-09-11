use crate::config::Config;
use crate::route::Route;
use crate::worker::WorkerOptions;
use crate::{DicomListener, Error, worker};
use rad_tools_common::{Start, Stop};
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::mpsc::Sender;
use tracing::{error, warn};

pub struct EndpointManager {
    listeners: Vec<DicomListener>,
    routes: Vec<Route>,
    max_stop_attempts: usize,
    senders: Vec<Sender<bool>>,
    futures: Vec<Pin<Box<dyn Future<Output = crate::Result<()>>>>>,
}

impl Debug for EndpointManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "EndpointManager:")?;
        write!(f, "  listeners: {:#?}", &self.listeners)?;
        write!(f, "  routes: {:#?}", &self.routes)?;
        write!(f, "  senders: {:#?}", self.senders.len())?;
        write!(f, "  futures: {:#?}", self.futures.len())
    }
}

impl From<Config> for EndpointManager {
    fn from(config: Config) -> Self {
        let listeners = config
            .listeners
            .iter()
            .map(DicomListener::from)
            .collect::<Vec<_>>();

        let mut routes = vec![];
        for cr in &config.routes {
            let listener = listeners.iter().find(|l| l.name() == cr.name);
            if listener.is_none() {
                continue;
            }
            let listener = listener.unwrap();
            let dir = PathBuf::from(match listener {
                DicomListener::Dcmtk(listener) => &listener.output,
            });
            let endpoints = config
                .endpoints
                .iter()
                .filter(|e| {
                    let tname = match e {
                        crate::config::Endpoint::Dicom(endpoint) => &endpoint.name,
                        crate::config::Endpoint::Dir(endpoint) => &endpoint.name,
                    };
                    cr.endpoints.contains(tname)
                })
                .collect::<Vec<_>>();
            let endpoints = endpoints
                .iter()
                .map(|endpoint| {
                    let te = (*endpoint).clone();
                    crate::endpoint::Endpoint::from(te)
                })
                .collect::<Vec<crate::endpoint::Endpoint>>();

            let route = Route { dir, endpoints };
            routes.push(route);
        }

        Self {
            listeners,
            routes,
            max_stop_attempts: config.manager.max_stop_attempts,
            senders: vec![],
            futures: vec![],
        }
    }
}

impl EndpointManager {
    pub fn is_running(&self) -> bool {
        !self.is_stopped()
    }

    pub fn is_stopped(&self) -> bool {
        self.senders.is_empty() && self.futures.is_empty()
    }
}

pub async fn start_with(
    mut manager: EndpointManager,
    options: WorkerOptions,
) -> crate::Result<EndpointManager> {
    if manager.is_running() {
        warn!("Endpoint manager already running, stopping it first:");
        warn!("{:?}", &manager);
        return Err(Error::EndpointManagerAlreadyStarted);
    }
    // Start all listeners
    for listener in &mut manager.listeners {
        listener.start()?;
    }
    // Start all workers
    for route in &manager.routes {
        let (tx, rx) = std::sync::mpsc::channel();
        let troute = route.clone();
        let worker = Box::pin(worker::start_with(troute, rx, options));
        manager.senders.push(tx);
        manager.futures.push(worker);
    }

    Ok(manager)
}

pub async fn stop(mut manager: EndpointManager) -> crate::Result<EndpointManager> {
    if manager.is_stopped() {
        return Err(Error::EndpointManagerNotStarted);
    }
    // Stop all listeners
    for listener in &mut manager.listeners {
        listener.stop()?;
    }
    // Stop all workers
    let mut errors = vec![true; manager.senders.len()];
    let mut has_error = false;
    for j in 0..manager.max_stop_attempts {
        for (i, sender) in &mut manager.senders.iter().enumerate() {
            if errors[i] {
                if let Err(e) = sender.send(true) {
                    error!("Unable to send a stop signal to a worker[{i}]: {e:#?}");
                    errors[i] = true;
                } else {
                    errors[i] = false;
                }
            }
        }
        has_error = errors.contains(&true);
        if !has_error {
            break;
        }
        if j == manager.max_stop_attempts - 1 {
            error!(
                "Failed to stop all workers after {} attempts",
                manager.max_stop_attempts
            );
            break;
        }
    }
    manager.senders.clear();
    manager.futures.clear();
    if has_error {
        Err(Error::EndpointManagerNotAllWorkersStopped)
    } else {
        Ok(manager)
    }
}
