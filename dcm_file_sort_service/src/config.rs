use crate::{Cli, Error};
use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;
use tracing::Level;
use tracing::metadata::ParseLevelError;

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
    /// The number of milliseconds a thread will sleep after moving data from the input directory to the output directory. After this delay, the thread will process new files in the input directory.
    pub wait_time_millisec: u64,
    /// The number of milliseconds a thread will sleep if a file can't be copied or removed because the file resource is being used by another process.
    pub io_timeout_millisec: u64,
    /// Number of times the service will try to copy a file if the input file resource is being used by another process.
    pub copy_attempts: u64,
    /// Number of times the service will try to remove a file
    pub remove_attempts: usize,
    /// Number of seconds between the last modified time and the current time before a file is consided deletable.
    pub mtime_delay_secs: i64,
}

impl Default for Other {
    fn default() -> Self {
        Self {
            wait_time_millisec: 500,
            io_timeout_millisec: 500,
            copy_attempts: 100,
            remove_attempts: 10,
            mtime_delay_secs: 10,
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

impl TryFrom<Cli> for Config {
    type Error = Error;

    fn try_from(cli: Cli) -> Result<Self, Self::Error> {
        if let Some(args) = cli.manual_args {
            let mut config = Config::default();
            config.paths.input_dir = PathBuf::from(args.input_dir);
            config.paths.output_dir = PathBuf::from(args.output_dir);
            config.paths.unknown_dir = PathBuf::from(args.unknown_dir);
            config.log.level = if args.trace {
                Level::TRACE
            } else if args.debug {
                Level::DEBUG
            } else if args.verbose {
                Level::INFO
            } else {
                Level::WARN
            };
            Ok(config)
        } else if let Some(config_path) = cli.config {
            let config_content =
                std::fs::read_to_string(config_path).expect("Failed to read the config file");
            let config: Config =
                toml::from_str(&config_content).expect("Failed to parse the config file");
            Ok(config)
        } else {
            Err(Error::ConfigFromCli)
        }
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
