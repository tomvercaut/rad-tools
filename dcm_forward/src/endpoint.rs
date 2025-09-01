use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

#[derive(Debug)]
pub enum Endpoint {
    Dicom(DicomStreamEndpoint),
    Dir(DirEndpoint),
}

#[derive(Debug)]
pub struct EndpointManager {
    // Directory where the input DICOM files are stored.
    pub dir: PathBuf,
    // List of endpoints to send the DICOM files to.
    pub endpoints: Vec<Endpoint>,
}
