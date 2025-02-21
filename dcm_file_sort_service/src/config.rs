use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;
use tracing::metadata::ParseLevelError;
use tracing::Level;

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

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Log {
    #[serde_as(as = "DisplayFromStr")]
    pub level: Level,
}

// use serde::ser::{SerializeStruct, Serializer};
//
// impl Serialize for Log {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut state = serializer.serialize_struct("Log", 1)?;
//         state.serialize_field("level", &self.level.to_string())?;
//         state.end()
//     }
// }

impl Default for Log {
    fn default() -> Self {
        Self { level: Level::WARN }
    }
}

impl Display for Log {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.level)
    }
}

impl FromStr for Log {
    type Err = ParseLevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = Level::from_str(s)?;
        Ok(Self { level: s })
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

impl Config {
    /// Ensures that the directories specified in the `paths` field of the `Config` struct exist.
    ///
    /// This function checks for the existence of the input, output, and unknown directories
    /// specified in the `Paths` struct. If any of these directories do not exist, it will create them.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If all directories exist or are successfully created.
    /// * `Err(std::io::Error)` - If an error occurs while creating any of the directories.
    pub fn create_dirs(&self) -> Result<(), std::io::Error> {
        if !self.paths.input_dir.exists() {
            std::fs::create_dir_all(&self.paths.input_dir)?;
        }
        if !self.paths.output_dir.exists() {
            std::fs::create_dir_all(&self.paths.output_dir)?;
        }
        if !self.paths.unknown_dir.exists() {
            std::fs::create_dir_all(&self.paths.unknown_dir)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_serialize() {
        let log = Log { level: Level::INFO };
        let serialized = serde_json::to_string(&log).expect("Serialization failed");
        assert_eq!(serialized, r#"{"level":"INFO"}"#);
    }

    #[test]
    fn test_log_deserialize() {
        let serialized = r#"{"level":"INFO"}"#;
        let deserialized: Log = serde_json::from_str(serialized).expect("Deserialization failed");
        assert_eq!(deserialized.level, Level::INFO);
    }

    #[test]
    fn test_log_default_serialization() {
        let log = Log::default();
        let serialized = serde_json::to_string(&log).expect("Serialization failed");
        assert_eq!(serialized, r#"{"level":"WARN"}"#);
    }

    #[test]
    fn test_log_default_deserialization() {
        let serialized = r#"{"level":"WARN"}"#;
        let deserialized: Log = serde_json::from_str(serialized).expect("Deserialization failed");
        assert_eq!(deserialized.level, Level::WARN);
    }

    #[test]
    fn test_log_invalid_deserialization() {
        let serialized = r#"{"level":"INVALID_LEVEL"}"#;
        let deserialization_result: Result<Log, _> = serde_json::from_str(serialized);
        assert!(deserialization_result.is_err());
    }
}
