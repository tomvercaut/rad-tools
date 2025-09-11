use crate::Error;
use rad_tools_common::Validate;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{error, warn};

/// Listener for DICOM files.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DicomListener {
    /// Unique name for the listener
    pub name: String,
    /// Port to listen on
    pub port: u16,
    /// DICOM AE Title
    pub ae: String,
    /// Output directory for DICOM files
    pub output: String,
}

/// Endpoint to send DICOM files to.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DicomStreamEndpoint {
    /// Unique name for the endpoint
    pub name: String,
    /// Address to send the DICOM files to
    pub addr: String,
    /// Port to send the DICOM files to
    pub port: u16,
    /// DICOM my calling AE Title
    pub aet: String,
    /// DICOM called AE Title
    pub aec: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DirEndpoint {
    /// Unique name for the endpoint
    pub name: String,
    /// Path to the directory to send the DICOM files to
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Endpoint {
    Dicom(DicomStreamEndpoint),
    Dir(DirEndpoint),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Route {
    pub name: String,
    pub endpoints: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Manager {
    /// Maximum number of attempts to stop all workers
    pub max_stop_attempts: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    pub listeners: Vec<DicomListener>,
    pub endpoints: Vec<Endpoint>,
    pub routes: Vec<Route>,
    pub manager: Manager,
}

impl Validate<crate::Result<()>> for Config {
    fn validate(&self) -> crate::Result<()> {
        if self.listeners.is_empty() {
            return Err(Error::NoListenersConfigured);
        }
        if self.endpoints.is_empty() {
            return Err(Error::NoEndpointsConfigured);
        }
        for endpoint in &self.endpoints {
            match endpoint {
                Endpoint::Dicom(_) => {}
                Endpoint::Dir(endpoint) => {
                    if !endpoint_path_exists(endpoint) {
                        return Err(Error::DirectoryEndpointPathDoesNotExist);
                    }
                }
            }
        }
        are_routes_linked(self)?;
        Ok(())
    }
}

fn endpoint_path_exists(endpoint: &DirEndpoint) -> bool {
    warn!("DirEndpoint path {} is not a directory", endpoint.path);
    Path::new(&endpoint.path).is_dir()
}

pub fn are_routes_linked(config: &Config) -> crate::Result<()> {
    for route in &config.routes {
        let has_listener = config
            .listeners
            .iter()
            .any(|listener| listener.name == route.endpoints[0]);
        let has_endpoint = config.endpoints.iter().any(|endpoint| match endpoint {
            Endpoint::Dicom(value) => value.name == route.endpoints[1],
            Endpoint::Dir(value) => value.name == route.endpoints[1],
        });
        if !has_listener || !has_endpoint {
            error!(
                "Route from {} ↦ {} is not linked.",
                route.endpoints[0], route.endpoints[1]
            );
            return Err(Error::RouteNotFound);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_routes() {
        let config = Config {
            listeners: vec![DicomListener {
                name: "listener1".to_string(),
                port: 104,
                ae: "DCM".to_string(),
                output: "/tmp".to_string(),
            }],
            endpoints: vec![Endpoint::Dicom(DicomStreamEndpoint {
                name: "endpoint1".to_string(),
                addr: "127.0.0.1".to_string(),
                port: 105,
                aet: "AET".to_string(),
                aec: "STORE".to_string(),
            })],
            routes: vec![Route {
                name: "route1".to_string(),
                endpoints: vec!["listener1".to_string(), "endpoint1".to_string()],
            }],
            manager: Manager {
                max_stop_attempts: 100,
            },
        };
        assert!(are_routes_linked(&config).is_ok());
    }

    #[test]
    fn test_invalid_route_listener() {
        let config = Config {
            listeners: vec![],
            endpoints: vec![Endpoint::Dicom(DicomStreamEndpoint {
                name: "endpoint1".to_string(),
                addr: "127.0.0.1".to_string(),
                port: 105,
                aet: "AET".to_string(),
                aec: "STORE".to_string(),
            })],
            routes: vec![Route {
                name: "route1".to_string(),
                endpoints: vec!["listener1".to_string(), "endpoint1".to_string()],
            }],
            manager: Manager {
                max_stop_attempts: 100,
            },
        };
        assert!(are_routes_linked(&config).is_err());
    }

    #[test]
    fn test_invalid_route_endpoint() {
        let config = Config {
            listeners: vec![DicomListener {
                name: "listener1".to_string(),
                port: 104,
                ae: "DCM".to_string(),
                output: "/tmp".to_string(),
            }],
            endpoints: vec![],
            routes: vec![Route {
                name: "route1".to_string(),
                endpoints: vec!["listener1".to_string(), "endpoint1".to_string()],
            }],
            manager: Manager {
                max_stop_attempts: 100,
            },
        };
        assert!(are_routes_linked(&config).is_err());
    }
}
