use crate::path_gen::DicomPathGeneratorType;
use crate::{Cli, Error};
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

/// Contains configuration for path generators used to determine the directory paths
/// for processed data. Each generator type defines specific rules for organizing files
/// in the output directory structure.
///
/// # Fields
///
///   - "dicom": Organizes DICOM files
///
/// * `unknown` - Specifies the generator type for unrecognized files. Supported values:
///   - "unknown": Handles files that cannot be processed as DICOM data
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct PathGenerators {
    /// Path generator for DICOM data (accepted value: "dicom_default", "dicom_uzg")
    pub dicom: DicomPathGeneratorType,
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
    /// Number of seconds between the last modified time and the current time before a file is considered sortable.
    pub mtime_delay_secs: i64,
    /// Limit the number of attempts to generate a unique filename in the output directory.
    /// If the creation time is available on the filesystem, this will also be taken into account.
    pub limit_unique_filenames: usize,
    /// Limit the number of files being added for processing. If the limit is reached, the files are first moved into their new directories before searching for more files.
    /// This prevents:
    /// - Searching / iterating through a large number of files
    /// - Avoids allocating memory to store all the DICOM-related metadata for all those files
    /// - Eligable data will be moved faster as a result
    pub limit_max_processed_files: usize,
}

impl Default for Other {
    fn default() -> Self {
        Self {
            wait_time_millisec: 500,
            io_timeout_millisec: 500,
            copy_attempts: 100,
            remove_attempts: 10,
            mtime_delay_secs: 10,
            limit_unique_filenames: 1000,
            limit_max_processed_files: 1000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Config {
    /// Paths required for reading and writing the data
    pub paths: Paths,
    /// Path generators
    #[serde(rename = "path_generators", default = "PathGenerators::default")]
    pub path_gens: PathGenerators,
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
        let config_content =
            std::fs::read_to_string(cli.config).expect("Failed to read the config file");
        let config: Config =
            toml::from_str(&config_content).expect("Failed to parse the config file");
        Ok(config)
    }
}
