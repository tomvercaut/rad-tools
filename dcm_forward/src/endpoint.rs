use serde::{Deserialize, Serialize};

/// Endpoint to send DICOM files to.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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

impl From<crate::config::DicomStreamEndpoint> for DicomStreamEndpoint {
    fn from(value: crate::config::DicomStreamEndpoint) -> Self {
        Self {
            name: value.name,
            addr: value.addr,
            port: value.port,
            aet: value.aet,
            aec: value.aec,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DirEndpoint {
    /// Unique name for the endpoint
    pub name: String,
    /// Path to the directory to send the DICOM files to
    pub path: String,
}

impl From<crate::config::DirEndpoint> for DirEndpoint {
    fn from(value: crate::config::DirEndpoint) -> Self {
        Self {
            name: value.name,
            path: value.path,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Endpoint {
    Dicom(DicomStreamEndpoint),
    Dir(DirEndpoint),
}

impl From<crate::config::Endpoint> for Endpoint {
    fn from(value: crate::config::Endpoint) -> Self {
        match value {
            crate::config::Endpoint::Dicom(value) => Endpoint::Dicom(value.into()),
            crate::config::Endpoint::Dir(value) => Endpoint::Dir(value.into()),
        }
    }
}

impl Endpoint {
    pub fn name(&self) -> &str {
        match self {
            Endpoint::Dicom(endpoint) => &endpoint.name,
            Endpoint::Dir(endpoint) => &endpoint.name,
        }
    }
}
