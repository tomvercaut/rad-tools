mod cli;
mod config;
pub mod path_gen;
pub mod service;
use crate::path_gen::{
    DicomDirPathGeneratorFactory, SortedDirPathGenerator, SortedPathGeneratorError,
};
pub use cli::{Cli, ENV_LOG};
pub use config::Config;
#[allow(unused_imports)]
use dicom_core::chrono::Datelike;
use dicom_core::chrono::NaiveDate;
use dicom_object::{ReadError, open_file};
use filetime::FileTime;
use rad_tools_common::fs::{
    DefaultUniquePathError, DefaultUniquePathGenerator, UniquePathGenerator,
};
use serde::{Deserialize, Serialize};
use std::ffi::{OsStr, OsString};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;
use tracing::{debug, error, info, trace, warn};
use walkdir::WalkDir;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Ord, PartialOrd)]
pub enum ServiceState {
    Running,
    RequestToStop,
    Stopped,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Read error: {0}")]
    DicomRead(#[from] ReadError),
    #[error("Walk directory error: {0}")]
    WalkDir(#[from] walkdir::Error),
    #[error("Invalid date format: unable to determine year")]
    InvalidDateFormatYear,
    #[error("Invalid date format: unable to determine a valid month")]
    InvalidDateFormatMonth,
    #[error("Invalid date format: unable to determine a valid day")]
    InvalidDateFormatDay,
    #[error("Invalid calendar date")]
    InvalidDateCalendar,
    #[error("Invalid date")]
    InvalidDate,
    #[error("Invalid date of birth format")]
    InvalidDateOfBirth,
    #[error("Unable to parse integer from string")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Unable to determine the parent of a file or directory")]
    UnknownParent,
    #[error("Unable to determine filename")]
    UnknownFilename,
    #[error("Unable to create config from Cli")]
    ConfigFromCli,
    #[error("Unable to get the last modified time from a file / path")]
    LastModifiedTime,
    #[error("Unable to create a unique file path")]
    DefaultUniqueUniqueFilePath(#[from] DefaultUniquePathError),
    #[error("Error while generating a file/directory path.")]
    PathGenerator(#[from] SortedPathGeneratorError),
    #[error("An error occurred while parsing JSON data: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("PathGeneratorType can't be created from String value.")]
    InvalidFromStrToPathGeneratorType,
}

pub type Result<T> = std::result::Result<T, Error>;

/// Data used to sort the DICOM file
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct DicomData {
    /// SOP Instance UID
    pub sop_instance_uid: String,
    /// Modality
    pub modality: String,
    /// Patient ID
    pub patient_id: String,
    /// Date of birth
    pub date_of_birth: Option<NaiveDate>,
    /// DICOM file to be sorted
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct UnknownData {
    /// Path to the unknown data
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum SortingData {
    #[default]
    None,
    Dicom(DicomData),
    Unknown(UnknownData),
}

///
/// Represents the copy of a file from an input path to an output path.
///
/// This struct is used to log and track the relocation of files during processing.
///
/// # Fields
/// * `input` - The original file path where the file is located.
/// * `output` - The destination file path where the file is copied to.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub(crate) struct CopiedData {
    /// Input file path
    pub input: PathBuf,
    /// Output file path
    pub output: PathBuf,
}

impl std::fmt::Display for CopiedData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Copied {} to {}",
            self.input.display(),
            self.output.display()
        )
    }
}

/// Runs the DICOM file sorting service in a continuous loop until a stop signal is received.
///
/// This function implements the main service loop that:
/// * Monitors a specified input directory for DICOM and unknown files
/// * Sorts and copies files to appropriate output locations based on their metadata
/// * Removes successfully processed files from the input directory
/// * Cleans up empty subdirectories in the input path
/// * Waits for a specified interval before starting the next processing cycle
///
/// The service continues running until it receives a non-Running state through the receiver channel.
///
/// # Arguments
/// * `config` - Configuration containing input/output paths and other settings
/// * `rx` - Channel receiver [`Receiver<ServiceState>`] for monitoring service state changes
/// * `wait_millisecs` - Milliseconds to wait between processing cycles
///
/// # Returns
/// * `Ok(())` - If the service completes successfully after receiving a stop signal
/// * `Err` - If an error occurs during file processing or directory operations
///
/// # Errors
/// This function may return errors from:
/// * File operations (copying, deleting)
/// * Directory operations (creating, removing)
/// * DICOM metadata extraction
/// * Invalid date formats
pub fn run_service(config: &Config, rx: Receiver<ServiceState>) -> Result<()> {
    let handle_copied_data = |r: Result<CopiedData>| -> Result<()> {
        match r {
            Ok(copied_data) => remove_file_retry_on_busy(
                copied_data.input,
                config.other.io_timeout_millisec,
                config.other.remove_attempts,
            ),
            Err(e) => {
                error!("Error copying data: {}", e);
                Err(e)
            }
        }
    };
    'outer: loop {
        let r = get_sorting_data(config, &rx);
        if let Err(e) = r {
            error!("Error sorting all the data: {}", e);
            return Err(e);
        }
        let (vdd, unknowns, stopped) = r?;
        if stopped {
            break 'outer;
        }
        // Process the DICOM data
        for dd in vdd {
            if should_stop(&rx) {
                break 'outer;
            }
            handle_copied_data(copy_dicom_data(dd, config))?;
        }
        // Process the unkown data
        for ud in unknowns {
            if should_stop(&rx) {
                break 'outer;
            }
            handle_copied_data(copy_unkown_data(ud, config))?;
        }
        if let Err(e) = remove_empty_sub_dirs(&config.paths.input_dir) {
            error!(
                "Error while trying to remove empty subdirectory [{}]: {}",
                &config.paths.input_dir.display(),
                e
            );
        }
        if should_stop(&rx) {
            break 'outer;
        }
        std::thread::sleep(std::time::Duration::from_millis(
            config.other.wait_time_millisec,
        ));
    }
    Ok(())
}

/// Attempts to remove a file with retries if the file is temporarily busy.
///
/// This function makes multiple attempts to remove a file, with a configurable delay between
/// attempts if the file is locked or in use by another process. This is particularly useful
/// when dealing with files that might be temporarily locked by system processes or other
/// applications.
///
/// # Arguments
/// * `path` - A path reference to the file that should be removed
/// * `wait_millisecs` - The number of milliseconds to wait between retry attempts
/// * `max_attempts` - The maximum number of removal attempts before giving up
///
/// # Returns
/// * `Ok(())` - If the file was successfully removed
/// * `Err(Error)` - If the file could not be removed after all attempts
///
/// # Errors
/// * Returns `Error::IO` with the underlying IO error if the file cannot be removed
/// * Returns `Error::IO` with a custom error message if all attempts are exhausted
///
/// # Behavior
/// * If the file removal fails with `ResourceBusy`, the function will wait for the specified
///   duration and retry
/// * For any other type of error, the function will return immediately with that error
/// * After the maximum number of attempts, returns an error indicating the operation failed
fn remove_file_retry_on_busy<P: AsRef<Path>>(
    path: P,
    wait_millisecs: u64,
    max_attempts: usize,
) -> Result<()> {
    for _ in 0..max_attempts {
        match std::fs::remove_file(path.as_ref()) {
            Ok(_) => {
                debug!("Removed file: {}", path.as_ref().display());
                return Ok(());
            }
            Err(e) => {
                error!(
                    "Unable to remove file: {}. Error: {}",
                    path.as_ref().display(),
                    e
                );
                if e.kind() == ErrorKind::ResourceBusy {
                    std::thread::sleep(std::time::Duration::from_millis(wait_millisecs));
                    continue;
                } else {
                    return Err(Error::IO(e));
                }
            }
        }
    }
    Err(Error::IO(std::io::Error::other(
        "Unable to remove file after {} attempts",
    )))
}

/// Checks if the service should stop processing based on its state.
///
/// This function attempts to receive a state update from the provided channel
/// without blocking. If a state other than `ServiceState::Running` is received,
/// it indicates that the service should stop.
///
/// # Arguments
/// * `rx` - A receiver that monitors the service state changes
///
/// # Returns
/// * `true` if the service should stop (received state is not `Running`)
/// * `false` if no state was received or if the received state is `Running`
fn should_stop(rx: &Receiver<ServiceState>) -> bool {
    if let Ok(state) = rx.try_recv()
        && state != ServiceState::Running
    {
        return true;
    }
    false
}

///
/// Iterates over the files in the given directory and gathers sorting data.
///
/// This function traverses the specified directory and its subdirectories, examining each
/// file to determine if it is a DICOM file. It attempts to extract metadata and collects
/// it as `DicomData`.
/// Files missing the essential DICOM fields will be logged with a warning and categorized as
/// unknown data (`SortingData::Unknown`).
/// The function monitors a Channel receiver to ensure it halts processing if the service
/// state is no longer `ServiceState::Running`.
///
/// # Arguments
/// * `config` - A `Config` struct providing the input directory path.
/// * `rx` - A receiver [`Receiver<ServiceState>`] to monitor changes in the state of the service.
///
/// # Returns
/// * `Ok((Vec<DicomData>, Vec<UnknownData>, bool)` - A tuple containing:
///     - a vector containing `DicomData` for identified DICOM files
///     - a vector containing `UnknownData` for files from which no DICOM data could be extracted
///     - a boolean indicating if the function was stopped while transversing the filedata.
/// * `Err` - If an error occurs during the directory traversal or file processing.
///
/// # Errors
/// This function returns an error if the DICOM file cannot be opened or read. Metadata extraction errors will not cause
/// a direct error result, but instead result in the file being considered as unknown data.
///
/// # Metadata Tags Retrieved
/// - `PatientID` (DICOM Tag: (0010,0020)): Identifies the patient.
/// - `PatientBirthDate` (DICOM Tag: (0010,0030)): The patient's date of birth. If missing, this field may attempt to derive a value
///   from the Patient ID based on predefined rules.
/// - `SOPInstanceUID` (DICOM Tag: (0008,0018)): A unique identifier for the specific object.
/// - `Modality` (DICOM Tag: (0008,0060)): Describes the type of equipment used.
///
/// # Processing Behavior
/// If critical metadata is successfully extracted, the file will be categorized as DICOM data (`SortingData::Dicom`).
/// Files missing essential fields will be logged with a warning and categorized as unknown data (`SortingData::Unknown`).
///
fn get_sorting_data(
    config: &Config,
    rx: &Receiver<ServiceState>,
) -> Result<(Vec<DicomData>, Vec<UnknownData>, bool)> {
    let mut dicom_dataset = vec![];
    let mut unknown_dataset = vec![];
    let mut stopped = false;
    // for entry in WalkDir::new(&input_dir).into_iter().filter_map(|r| r.ok()) {
    for entry in WalkDir::new(&config.paths.input_dir) {
        match entry {
            Ok(entry) => {
                trace!("Processing file: {}", entry.path().display());
                if should_stop(rx) {
                    info!("Stopping the processing cycle");
                    stopped = true;
                    break;
                }
                let path = entry.path();

                if !path.is_file() {
                    continue;
                }
                let mtime = last_modified_time(path);
                if mtime.is_err() {
                    trace!(
                        "Skipping file: {} (last modified time not available)",
                        path.display()
                    );
                    continue;
                }
                let mtime = mtime?;
                let current_time = FileTime::now();
                if current_time.seconds() - mtime.seconds() < config.other.mtime_delay_secs {
                    trace!(
                        "Skipping file: {} (last modified less than {} seconds ago)",
                        path.display(),
                        config.other.mtime_delay_secs
                    );
                    continue;
                }
                match extract_dicom_metadata(path) {
                    Ok(sorting_data) => match sorting_data {
                        SortingData::Dicom(data) => {
                            dicom_dataset.push(data);
                        }
                        SortingData::Unknown(data) => {
                            unknown_dataset.push(data);
                        }
                        SortingData::None => {}
                    },
                    Err(e) => {
                        error!("Failed to process file {}: {}", path.display(), e);
                    }
                }
            }
            Err(e) => {
                error!("Failed to traverse directory: {}", e);
                continue;
            }
        }
    }
    Ok((dicom_dataset, unknown_dataset, stopped))
}

/// Gets the last modification time of a file.
///
/// This function retrieves the last modification timestamp of the specified file path.
/// If the file's metadata cannot be accessed, it returns an error.
///
/// # Arguments
/// * `path` - A path reference to the file whose modification time is being queried.
///
/// # Returns
/// * `Ok(FileTime)` - The last modification time of the file as a `FileTime` value.
/// * `Err(Error)` - If the file metadata cannot be accessed or read.
///
/// # Errors
/// Returns `Error::LastModifiedTime` if:
/// * The file does not exist
/// * The process lacks permissions to read the file metadata
/// * Other system-level errors occur while accessing the file
fn last_modified_time<P>(path: P) -> Result<FileTime>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let meta = std::fs::metadata(path);
    if meta.is_err() {
        trace!("Failed to get metadata for a file: {:#?}", path);
        return Err(Error::LastModifiedTime);
    }
    let meta = meta?;
    Ok(FileTime::from_last_modification_time(&meta))
}

///
/// Recursively removes empty subdirectories within the specified directory.
///
/// This function traverses the given directory and its subdirectories, identifying
/// directories that are empty and removing them. It ensures that no non-empty
/// directories are removed, and it operates silently on errors, logging them
/// when necessary.
///
/// # Arguments
/// * `dir` - The root directory to scan for empty subdirectories.
///
/// # Errors
/// Returns an error if there is an issue accessing the directory or removing
/// a directory. Errors during traversal, such as permission issues, are also
/// propagated.
///
/// # Behavior
/// * Empty subdirectories are removed.
/// * Directories containing at least one file or subdirectory are left untouched.
fn remove_empty_sub_dirs<P: AsRef<Path>>(dir: P) -> Result<()> {
    for entry in WalkDir::new(&dir).into_iter().filter_map(|r| r.ok()) {
        let path = entry.path();
        if path == dir.as_ref() {
            continue;
        }
        if path.is_dir() && is_dir_empty(path) {
            std::fs::remove_dir_all(path)?;
        }
    }
    Ok(())
}

fn remove_null_chars(s: &str) -> String {
    s.chars().filter(|c| *c != '\0').collect()
}

///
/// Extracts metadata from a DICOM file, specifically the Patient ID and Date of Birth.
/// If the birthday doesn't exist and the patient ID is 8 characters or longer, the 3, 4, 5, and 7th character are used as a substitue.
///
/// # Arguments
/// * `file_path` - A path to the DICOM file to be processed.
///
/// # Returns
/// * `Ok(Some(SortingData))` - If both Patient ID and Date of Birth are successfully extracted.
/// * `Ok(None)` - If the required metadata is missing or cannot be extracted.
/// * `Err` - If the DICOM file cannot be opened or read.
///
/// # Errors
/// This function returns an error if there is an issue opening the DICOM file or reading its
/// contents. Individual metadata extraction errors are logged but do not cause the function
/// to return an error.
///
/// The extracted metadata is represented as a `SortingData` struct containing the following:
/// * `patient_id` - The Patient ID (DICOM Tag: (0010,0020)) as a string.
/// * `date_of_birth` - The Date of Birth (DICOM Tag: (0010,0030)) as a string.
fn extract_dicom_metadata<P: AsRef<Path>>(file_path: P) -> Result<SortingData> {
    let dicom_file = open_file(file_path.as_ref());
    if dicom_file.is_err() {
        warn!(
            "Failed to extract metadata from file: {}\nThis file will not be treated as a DICOM file in future processing.",
            file_path.as_ref().display()
        );
        return Ok(SortingData::Unknown(UnknownData {
            path: file_path.as_ref().to_path_buf(),
        }));
    }
    let dicom_file = dicom_file?;

    let sop_instance_uid = dicom_file
        .element_by_name_opt("SOPInstanceUID")
        .map(|elem| elem.map(|elem| remove_null_chars(elem.string().unwrap().trim())));

    let modality = dicom_file
        .element_by_name_opt("Modality")
        .map(|elem| elem.map(|elem| remove_null_chars(elem.string().unwrap().trim())));

    // .map(|elem| remove_null_chars(elem.string().unwrap().trim()));

    // Extracting Patient ID (Tag: (0010,0020)) and Date of Birth (Tag: (0010,0030)).
    let patient_id = dicom_file
        .element_by_name_opt("PatientID")
        .map(|elem| elem.map(|elem| remove_null_chars(elem.string().unwrap().trim())));
    // .map(|elem| remove_null_chars(elem.string().unwrap().trim()));

    // Not required by DICOM
    let mut date_of_birth = dicom_file
        .element_by_name("PatientBirthDate")
        .ok()
        .and_then(|elem| match elem.string() {
            Ok(s) => parse_date(s.trim()).ok(),
            Err(_) => None,
        });

    // Extract the date of birth from the patient ID
    // This may not work in every case because there is no general consensus for this.
    let no_dob = date_of_birth.is_none();
    if no_dob
        && let Ok(Some(pid)) = patient_id.as_ref()
        && pid.len() >= 6
    {
        let t = format!("00{}", &pid[0..6]);
        if let Ok(date) = parse_date(&t) {
            date_of_birth = Some(date);
        }
    }

    debug!(
        "Extracting metadata from file: {:#?}\nSopInstanceUid: {:#?}\nPatientId: {:#?}\nModality: {:#?}\nDateOfBirth: {:#?}",
        &file_path.as_ref(),
        sop_instance_uid
            .as_ref()
            .unwrap_or(&Some("Error".to_string())),
        &patient_id,
        &modality,
        &date_of_birth
    );
    match (sop_instance_uid, patient_id, modality, date_of_birth) {
        (
            Ok(Some(sop_instance_uid)),
            Ok(Some(patient_id)),
            Ok(Some(modality)),
            Some(date_of_birth),
        ) => {
            debug!(
                "Extracted metadata from file: {}.\nSOP Instance UID: {}\nPatient ID: {}\nModality: {}\nDate of Birth: {}",
                file_path.as_ref().display(),
                sop_instance_uid,
                patient_id,
                modality,
                date_of_birth
            );
            Ok(SortingData::Dicom(DicomData {
                sop_instance_uid: sop_instance_uid.to_string(),
                modality: modality.to_string(),
                patient_id: patient_id.to_string(),
                date_of_birth: Some(date_of_birth),
                path: file_path.as_ref().to_path_buf(),
            }))
        }
        _ => {
            warn!(
                "Failed to extract metadata from file: {}\nThis file will not be treated as a DICOM file in future processing.",
                file_path.as_ref().display()
            );
            Ok(SortingData::Unknown(UnknownData {
                path: file_path.as_ref().to_path_buf(),
            }))
        }
    }
}

/// Parses a date string in YYYYMMDD format and validates it as a valid calendar date.
///
/// This function takes an 8-character string representing a date and attempts to convert
/// it into a `NaiveDate` object while performing various validation checks.
///
/// # Arguments
/// * `s` - A string slice that should contain exactly 8 characters in YYYYMMDD format
///
/// # Returns
/// * `Ok(NaiveDate)` - If the string is successfully parsed into a valid date
/// * `Err(Error)` - If the string is invalid or represents an impossible date
///
/// # Errors
/// * Returns `Error::InvalidDate` if the input string is not exactly 8 characters
/// * Returns `Error::InvalidDateFormatYear` if the year portion cannot be parsed
/// * Returns `Error::InvalidDateFormatMonth` if the month is not between 1 and 12
/// * Returns `Error::InvalidDateFormatDay` if the day is not between 1 and 31
/// * Returns `Error::InvalidDateCalendar` if the combination of year, month, and day is not a valid calendar date
pub fn parse_date(s: &str) -> Result<NaiveDate> {
    if s.len() != 8 {
        return Err(Error::InvalidDate);
    }

    let year = match s[0..4].parse::<i32>() {
        Ok(year) => Ok(year),
        Err(_) => Err(Error::InvalidDateFormatYear),
    }?;
    let month = s[4..6].parse::<i32>()?;
    let day = s[6..].parse::<i32>()?;
    if month <= 0 || month > 12 {
        return Err(Error::InvalidDateFormatMonth);
    }
    if day <= 0 || day > 31 {
        return Err(Error::InvalidDateFormatDay);
    }
    match NaiveDate::from_ymd_opt(year, month as u32, day as u32) {
        None => Err(Error::InvalidDateCalendar),
        Some(date) => Ok(date),
    }
}

/// Copy a file from a source path to a destination path.
///
/// A retry mechanism is implemented to handle cases where the file cannot be copied due to temporary issues such as the target file or source being locked or busy. It ensures reliable file copying in such scenarios by retrying the operation for a configurable number of attempts.
///
/// # Arguments
/// * `input` - source file path to be copied
/// * `output` - destination file path where the file will be copied
/// * `config` - configuration settings
///
/// # Returns
/// * `Ok(CopiedData)` - If the file is successfully copied to the destination.
///     - `CopiedData` includes the `input` (source path) and `output` (destination path) to help log and track the operation.
/// * `Err(Error)` - If the file cannot be copied after all retry attempts or if any non-temporary issue is encountered during the copy operation.
fn copy_with_retry_on_busy<P: AsRef<Path>>(
    input: P,
    output: P,
    config: &Config,
) -> Result<CopiedData> {
    let mut last_error = std::io::Error::other("");
    for _ in 0..config.other.copy_attempts {
        // Copy the file to the destination path
        match std::fs::copy(input.as_ref(), output.as_ref()) {
            Ok(_) => {
                return Ok(CopiedData {
                    input: input.as_ref().to_path_buf(),
                    output: output.as_ref().to_path_buf(),
                });
            }
            Err(e) => {
                if e.kind() != ErrorKind::ResourceBusy {
                    error!(
                        "Failed to copy file: {} -> {}: {}",
                        input.as_ref().display(),
                        output.as_ref().display(),
                        e
                    );
                    return Err(e.into());
                }
                last_error = e;
                trace!(
                    "Resource busy error, retrying in {} milliseconds ...",
                    config.other.io_timeout_millisec
                );
                std::thread::sleep(std::time::Duration::from_millis(
                    config.other.io_timeout_millisec,
                ));
            }
        }
    }
    Err(Error::IO(last_error))
}

/// Copies a DICOM file to a designated output directory based on its metadata.
///
/// # Arguments
/// * `data` - A `SortingData` struct containing metadata (e.g., Patient ID and Date of Birth) and the original file path.
/// * `config` - A `Config` struct providing the output directory path.
///
/// # Returns
/// * `Ok(CopiedData)` - If the file is successfully copied to the output directory.
/// * `Err` - If any error occurs during the directory creation, file copying, or if the metadata does not adhere to expected formats.
///
/// # Errors
/// * Returns an error if the output directory cannot be created.
/// * Returns an error if the provided date of birth format is invalid (not in `YYYYMMDD` format).
/// * Returns an error if the source file name is invalid or missing.
///
/// # Behavior
/// * Organizes files in the output directory using the patient's date of birth (as a `MMDD` folder structure)
///   and their Patient ID.
/// * If the specified directories in the output path do not exist, they are created automatically.
/// * Uses the `tracing` crate to log any errors, such as invalid date of birth format or issues with file copying.
///
/// # Assumptions
/// * `date_of_birth` in `SortingData` is expected to be in the format `YYYYMMDD`. Non-matching formats will result in an error.
///
fn copy_dicom_data(data: DicomData, config: &Config) -> Result<CopiedData> {
    let source_path = &data.path;

    let sub_dir =
        DicomDirPathGeneratorFactory::new(config.path_gens.dicom, &data).sort_dir_path()?;

    let output_path = config.paths.output_dir.join(sub_dir);

    // Create the necessary directories if they do not already exist
    debug!("Creating output directory: {}", output_path.display());
    if let Err(e) = std::fs::create_dir_all(&output_path)
        && e.kind() != ErrorKind::AlreadyExists
    {
        return Err(e.into());
    }

    // Build a unique filename
    let mut name = OsString::from(&data.modality);
    name.push(".");
    name.push(&data.sop_instance_uid);

    let extension = OsStr::new("dcm");

    let generator = DefaultUniquePathGenerator {
        dir: output_path,
        name: name.as_os_str(),
        extension: Some(extension),
        limit: config.other.limit_unique_filenames,
    };

    // Construct the final file path in the output directory
    let dest_file_path = generator.get_unique_path()?;

    debug!(
        "Copying file: {} -> {}",
        source_path.display(),
        dest_file_path.display()
    );
    // Copy the file to the destination path
    copy_with_retry_on_busy(source_path, &dest_file_path, config)
}

/// Copies an unknown data file to a designated output directory.
///
/// # Arguments
///
/// * `data` - An `UnknownData` struct containing metadata and the original file path.
/// * `config` - A `Config` struct providing paths, including the directory for unknown files.
///
/// # Returns
///
/// * `Ok(CopiedData)` - If the file is successfully copied to the output directory.
/// * `Err` - If any error occurs during the file copying process.
///
/// # Errors
///
/// * Returns an error if the source file cannot be copied to the destination unknown directory.
///
/// # Behavior
///
/// * Copies the file from its original path to the unknown directory path configured in `Config`.
/// * Logs debug information about the file copy operation, including source and destination paths.
fn copy_unkown_data(data: UnknownData, config: &Config) -> Result<CopiedData> {
    // Construct the final file path in the output directory
    let file_stem = data.path.file_stem().ok_or(Error::UnknownFilename)?;
    let extension = data.path.extension();
    let generator = DefaultUniquePathGenerator {
        dir: config.paths.unknown_dir.clone(),
        name: file_stem,
        extension,
        limit: config.other.limit_unique_filenames,
    };
    let dest_file_path = generator.get_unique_path()?;

    debug!(
        "Copying file: {} -> {}",
        data.path.display(),
        dest_file_path.display()
    );
    // Copy the file to the destination path
    copy_with_retry_on_busy(data.path, dest_file_path, config)
}

/// Checks whether a given directory is empty.
///
/// This function takes a path reference and determines if the directory
/// contains any entries. If the directory exists and contains no entries,
/// it returns `true`. Otherwise, it returns `false`.
///
/// # Arguments
///
/// * `dir` - A path reference to the directory to check.
///
/// # Returns
///
/// * `true` if the directory is empty or does not contain any entries.
/// * `false` if the directory contains at least one entry or if an error occurs while reading the directory.
fn is_dir_empty<P: AsRef<Path>>(dir: P) -> bool {
    if let Ok(entries) = std::fs::read_dir(dir) {
        return entries.count() == 0;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{PathGenerators, Paths};
    use crate::path_gen::DicomPathGeneratorType;
    use dicom_core::{DataElement, PrimitiveValue};
    use dicom_dictionary_std::tags;
    use dicom_dictionary_std::uids::CT_IMAGE_STORAGE;
    use dicom_object::{FileMetaTableBuilder, InMemDicomObject};
    use tempfile::TempDir;

    fn create_test_dicom_file(
        patient_id: &str,
        date_of_birth: &str,
        output_file_path: &Path,
    ) -> Result<()> {
        let mut obj = InMemDicomObject::new_empty();

        obj.put(DataElement::new(
            tags::SOP_INSTANCE_UID,
            dicom_core::VR::UI,
            PrimitiveValue::from("1.2.40.0.13.1.224652156348011029364280439841088994139"),
        ));

        obj.put(DataElement::new(
            tags::MODALITY,
            dicom_core::VR::CS,
            PrimitiveValue::from("CT"),
        ));

        // Add Patient ID
        obj.put(DataElement::new(
            tags::PATIENT_ID,
            dicom_core::VR::LO,
            PrimitiveValue::from(patient_id.to_string()),
        ));

        // Add Patient Date of Birth
        obj.put(DataElement::new(
            tags::PATIENT_BIRTH_DATE,
            dicom_core::VR::DA,
            PrimitiveValue::from(date_of_birth.to_string()),
        ));

        // Write the DICOM object to a file
        let file_obj = obj
            .with_meta(
                FileMetaTableBuilder::new()
                    .transfer_syntax(dicom_transfer_syntax_registry::default().erased().uid())
                    .media_storage_sop_class_uid(CT_IMAGE_STORAGE),
            )
            .unwrap();
        file_obj.write_to_file(output_file_path).unwrap();
        Ok(())
    }

    fn get_test_config(temp_dir: &TempDir) -> Config {
        let tp = temp_dir.path();
        let config = Config {
            paths: Paths {
                input_dir: tp.join("input"),
                output_dir: tp.join("output"),
                unknown_dir: tp.join("unknown"),
            },
            path_gens: Default::default(),
            other: Default::default(),
        };
        config.create_dirs().unwrap();
        config
    }

    fn get_test_config_uzg(temp_dir: &TempDir) -> Config {
        let tp = temp_dir.path();
        let config = Config {
            paths: Paths {
                input_dir: tp.join("input"),
                output_dir: tp.join("output"),
                unknown_dir: tp.join("unknown"),
            },
            path_gens: PathGenerators {
                dicom: DicomPathGeneratorType::Uzg,
            },
            other: Default::default(),
        };
        config.create_dirs().unwrap();
        config
    }

    fn get_sorting_data(patient_id: &str, input_file: &Path) -> DicomData {
        DicomData {
            sop_instance_uid: "9.3.12.2.1107.5.1.7.130037.30000025021708505036500000024"
                .to_string(),
            modality: "CT".to_string(),
            patient_id: patient_id.to_string(),
            date_of_birth: Some(NaiveDate::from_ymd_opt(1985, 6, 15).unwrap()),
            path: input_file.into(),
        }
    }

    #[test]
    fn test_copy_dicom_data_success_default_generator() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config = get_test_config(&temp_dir);

        let patient_id = "12345";
        let date_of_birth = "19850615";
        let input_file = config.paths.input_dir.join("test.dcm");

        create_test_dicom_file(patient_id, date_of_birth, &input_file)
            .expect("Unable to create a test DICOM file.");

        let sorting_data = get_sorting_data(patient_id, &input_file);

        let output_file_name = format!(
            "{}.{}.dcm",
            &sorting_data.modality, &sorting_data.sop_instance_uid
        );

        // Call the function
        let copied_data = copy_dicom_data(sorting_data, &config).unwrap();

        // Validate that the file was copied to the correct output directory
        let expected_output_path = config
            .paths
            .output_dir
            .join("patient_id")
            .join(patient_id)
            .join(output_file_name);
        debug!("Expected output path: {}", expected_output_path.display());
        debug!("Actual output path: {}", copied_data.output.display());
        assert!(copied_data.output.exists());
        // assert!(expected_output_path.exists());

        // Validate the channel message
        assert_eq!(copied_data.input, input_file);
        assert_eq!(copied_data.output, expected_output_path);
        std::fs::remove_dir_all(config.paths.input_dir.parent().unwrap()).unwrap()
    }

    #[test]
    fn test_copy_dicom_data_success_uzg_generator() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config = get_test_config_uzg(&temp_dir);

        let patient_id = "12345";
        let date_of_birth = "19850615";
        let input_file = config.paths.input_dir.join("test.dcm");

        create_test_dicom_file(patient_id, date_of_birth, &input_file)
            .expect("Unable to create a test DICOM file.");

        let sorting_data = get_sorting_data(patient_id, &input_file);

        let output_file_name = format!(
            "{}.{}.dcm",
            &sorting_data.modality, &sorting_data.sop_instance_uid
        );

        // Call the function
        let copied_data = copy_dicom_data(sorting_data, &config).unwrap();

        // Validate that the file was copied to the correct output directory
        let expected_output_path = config
            .paths
            .output_dir
            .join(&date_of_birth[4..])
            .join(patient_id)
            .join(output_file_name);
        debug!("Expected output path: {}", expected_output_path.display());
        debug!("Actual output path: {}", copied_data.output.display());
        assert!(copied_data.output.exists());
        // assert!(expected_output_path.exists());

        // Validate the channel message
        assert_eq!(copied_data.input, input_file);
        assert_eq!(copied_data.output, expected_output_path);
        std::fs::remove_dir_all(config.paths.input_dir.parent().unwrap()).unwrap()
    }

    #[test]
    fn test_copy_dicom_data_missing_source_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config = get_test_config(&temp_dir);
        let patient_id = "12345";
        let input_file = config.paths.input_dir.join("test.dcm");

        let sorting_data = get_sorting_data(patient_id, &input_file);

        // Call the function
        let result = copy_dicom_data(sorting_data, &config);

        // Validate that the function returns an error
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_date_valid() {
        let result = parse_date("20230901").unwrap();
        assert_eq!(result.year(), 2023);
        assert_eq!(result.month(), 9);
        assert_eq!(result.day(), 1);
    }

    #[test]
    fn test_parse_date_invalid_year() {
        let result = parse_date("abcd0901");
        assert!(matches!(result, Err(Error::InvalidDateFormatYear)));
    }

    #[test]
    fn test_parse_date_invalid_month() {
        let result = parse_date("20231301");
        assert!(matches!(result, Err(Error::InvalidDateFormatMonth)));
    }

    #[test]
    fn test_parse_date_invalid_day() {
        let result = parse_date("20230932");
        assert!(matches!(result, Err(Error::InvalidDateFormatDay)));
    }

    #[test]
    fn test_parse_date_invalid_length() {
        let result = parse_date("2023090");
        assert!(matches!(result, Err(Error::InvalidDate)));
    }

    #[test]
    fn test_parse_date_invalid_calendar_date() {
        let result = parse_date("20230931");
        assert!(matches!(result, Err(Error::InvalidDateCalendar)));
    }
}
