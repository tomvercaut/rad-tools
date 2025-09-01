use serde::{Deserialize, Serialize};
use tracing::error;

/// Listener for DICOM files.
#[derive(Debug, Default, Serialize, Deserialize)]
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
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DicomStreamEndpoint {
    /// Unique name for the endpoint
    pub name: String,
    /// Address to send the DICOM files to
    pub addr: String,
    /// Port to send the DICOM files to
    pub port: u16,
    /// DICOM AE Title
    pub ae: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DirEndpoint {
    /// Unique name for the endpoint
    pub name: String,
    /// Path to the directory to send the DICOM files to
    pub path: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Route {
    pub name: String,
    pub endpoints: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub listeners: Vec<DicomListener>,
    pub endpoints: Vec<DicomStreamEndpoint>,
    pub dir_endpoints: Vec<DirEndpoint>,
    pub routes: Vec<Route>,
}

pub fn are_routes_linked(config: &Config) -> bool {
    for route in &config.routes {
        let has_listener = config
            .listeners
            .iter()
            .any(|listener| listener.name == route.endpoints[0]);
        let has_endpoint = config
            .endpoints
            .iter()
            .any(|endpoint| endpoint.name == route.endpoints[1]);
        if !has_listener || !has_endpoint {
            error!(
                "Route from {} ↦ {} is not linked.",
                route.endpoints[0], route.endpoints[1]
            );
            return false;
        }
    }
    true
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
            endpoints: vec![DicomStreamEndpoint {
                name: "endpoint1".to_string(),
                addr: "127.0.0.1".to_string(),
                port: 105,
                ae: "STORE".to_string(),
            }],
            routes: vec![Route {
                name: "route1".to_string(),
                endpoints: vec!["listener1".to_string(), "endpoint1".to_string()],
            }],
            dir_endpoints: vec![],
        };
        assert!(are_routes_linked(&config));
    }

    #[test]
    fn test_invalid_route_listener() {
        let config = Config {
            listeners: vec![],
            endpoints: vec![DicomStreamEndpoint {
                name: "endpoint1".to_string(),
                addr: "127.0.0.1".to_string(),
                port: 105,
                ae: "STORE".to_string(),
            }],
            routes: vec![Route {
                name: "route1".to_string(),
                endpoints: vec!["listener1".to_string(), "endpoint1".to_string()],
            }],
            dir_endpoints: vec![],
        };
        assert!(!are_routes_linked(&config));
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
            dir_endpoints: vec![],
        };
        assert!(!are_routes_linked(&config));
    }
}
