use crate::DicomListener;
use crate::config::{Config, Route};
use crate::endpoint::Endpoint;
use rad_tools_common::Start;

#[derive(Debug)]
pub struct EndpointManager {
    listeners: Vec<DicomListener>,
    endpoints: Vec<Endpoint>,
    routes: Vec<Route>,
}

impl TryFrom<&Config> for EndpointManager {
    type Error = crate::Error;

    fn try_from(config: &Config) -> Result<Self, Self::Error> {
        let routes = Vec::new(); // Initialize empty and populate based on config later

        let listeners = config
            .listeners
            .iter()
            .map(|listener| DicomListener::from(listener))
            .collect();

        Ok(Self {
            listeners,
            endpoints: Vec::new(),
            routes,
        })
    }
}

impl Start<crate::Result<()>> for EndpointManager {
    fn start(&mut self) -> crate::Result<()> {
        for listener in &mut self.listeners {
            listener.start()?;
        }
        todo!()
    }
}
