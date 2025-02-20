use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Directories where the data is read from and written to.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Paths {
    /// Input directory where the DICOM data is initially stored
    pub input_dir: PathBuf,
    /// Output directory where the DICOM data is moved to.
    pub output_dir: PathBuf,
    /// Data that can't be processed will be moved into this directory.
    pub unknown_dir: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Log {
    pub level: String,
}

impl Default for Log {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Other {
    pub wait_time_millisec: u64,
}

impl Default for Other {
    fn default() -> Self {
        Self {
            wait_time_millisec: 500,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Config {
    /// Paths required for reading and writing the data
    pub paths: Paths,
    /// Logging configuration
    pub log: Log,
    /// Other config
    pub other: Other,
}
