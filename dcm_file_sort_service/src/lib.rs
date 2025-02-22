mod cli;
mod config;

pub use cli::Cli;
pub use config::Config;

use crate::Error::InvalidDateOfBirth;
use dicom_object::{open_file, ReadError};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
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
    #[error("Invalid date of birth format")]
    InvalidDateOfBirth,
    #[error("Unable to parse integer from string")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Unable to determine filename")]
    UnknownFilename,
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
    pub date_of_birth: String,
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

pub fn run_service(
    config: &Config,
    state: Arc<RwLock<ServiceState>>,
    wait_millisecs: u64,
) -> Result<()> {
    'outer: loop {
        let (vdd, unknowns) = get_sorting_data(config, state.clone())?;
        let handle_copied_data = |r: Result<CopiedData>| -> Result<()> {
            match r {
                Ok(copied_data) => match std::fs::remove_file(&copied_data.input) {
                    Ok(_) => {
                        debug!("Removed file: {}", copied_data.input.display());
                        Ok(())
                    }
                    Err(e) => Err(Error::IO(e)),
                },
                Err(e) => Err(e),
            }
        };
        // Process the DICOM data
        for dd in vdd {
            if should_stop(&state) {
                break 'outer;
            }
            handle_copied_data(copy_dicom_data(dd, config))?;
        }
        // Process the unkown data
        for ud in unknowns {
            if should_stop(&state) {
                break 'outer;
            }
            handle_copied_data(copy_unkown_data(ud, config))?;
        }
        remove_empty_sub_dirs(&config.paths.input_dir)?;
        if should_stop(&state) {
            break 'outer;
        }
        std::thread::sleep(std::time::Duration::from_millis(wait_millisecs));
    }
    Ok(())
}

fn should_stop(state: &Arc<RwLock<ServiceState>>) -> bool {
    if let Ok(inner) = state.try_read() {
        if *inner != ServiceState::Running {
            return true;
        }
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
/// unkown data (`SortingData::Unknown`).
/// The function monitors the shared `state` to ensure it halts processing if the service
/// state is no longer `ServiceState::Running`.
///
/// # Arguments
/// * `config` - A `Config` struct providing the input directory path.
/// * `state` - A shared atomic state indicating whether the service should continue running.
///
/// # Returns
/// * `Ok(Vec<SortingData>)` - A vector containing `SortingData` for identified DICOM files.
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
    state: Arc<RwLock<ServiceState>>,
) -> Result<(Vec<DicomData>, Vec<UnknownData>)> {
    let mut dicom_dataset = vec![];
    let mut unknown_dataset = vec![];
    // for entry in WalkDir::new(&input_dir).into_iter().filter_map(|r| r.ok()) {
    for entry in WalkDir::new(&config.paths.input_dir) {
        match entry {
            Ok(entry) => {
                trace!("Processing file: {}", entry.path().display());
                if should_stop(&state) {
                    info!("Stopping processing cycle");
                    break;
                }
                let path = entry.path();

                if !path.is_file() {
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
    Ok((dicom_dataset, unknown_dataset))
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
/// If the birthday doesn't exist and the patient ID is 8 characters or longer, the 3, 4, 5 and 7th character are used as a substitue.
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
        warn!("Failed to extract metadata from file: {}\nThis file will not be treated as a DICOM file in future processing.", file_path.as_ref().display());
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
    let date_of_birth = dicom_file
        .element_by_name("PatientBirthDate")
        .ok()
        .and_then(|elem| match elem.string() {
            Ok(s) => Some(s.trim().to_string()),
            Err(e) => {
                error!("Failed to extract Date of Birth: {}.\nTrying to extract part of the patient ID.", e);
                if let Ok(Some(pid)) = patient_id.as_ref() {
                    let t = format!("00{}", &pid[0..6]);
                    return Some(remove_null_chars(&t));
                }
                None
            }
        });

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
                date_of_birth: date_of_birth.to_string(),
                path: file_path.as_ref().to_path_buf(),
            }))
        }
        _ => {
            warn!("Failed to extract metadata from file: {}\nThis file will not be treated as a DICOM file in future processing.", file_path.as_ref().display());
            Ok(SortingData::Unknown(UnknownData {
                path: file_path.as_ref().to_path_buf(),
            }))
        }
    }
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
    // Extract the date of birth and patient ID from SortingData
    let dob = &data.date_of_birth;
    let patient_id = &data.patient_id;
    let source_path = &data.path;

    // Ensure date of birth is in valid format (YYYYMMDD). In case of invalid format, return an error.
    if dob.len() != 8 {
        error!("Invalid date of birth format: {}", dob);
        return Err(InvalidDateOfBirth);
    }

    let month_day = &dob[4..]; // Extract MMDD part
    let month = month_day[0..2].parse::<i32>()?;
    let day = month_day[2..].parse::<i32>()?;
    if month <= 0 || month > 12 || day <= 0 || day > 31 {
        error!("Invalid date of birth format: {}", dob);
        return Err(InvalidDateOfBirth);
    }
    let output_path =
        config
            .paths
            .output_dir
            .join(format!("{}/{}", month_day.trim(), patient_id.trim()));

    // Create the necessary directories if they do not already exist
    debug!("Creating output directory: {}", output_path.display());
    if let Err(e) = std::fs::create_dir_all(&output_path) {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            return Err(e.into());
        }
    }

    // Construct the final file path in the output directory
    let dest_file_path =
        output_path.join(format!("{}.{}.dcm", &data.modality, &data.sop_instance_uid));

    debug!(
        "Copying file: {} -> {}",
        source_path.display(),
        dest_file_path.display()
    );
    // Copy the file to the destination path
    std::fs::copy(source_path, &dest_file_path)?;

    Ok(CopiedData {
        input: data.path,
        output: dest_file_path,
    })
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
    let filename = data.path.file_name();
    if filename.is_none() {
        return Err(Error::UnknownFilename);
    }
    let filename = filename.unwrap();
    let dest_file_path = config.paths.unknown_dir.join(filename);

    debug!(
        "Copying file: {} -> {}",
        data.path.display(),
        dest_file_path.display()
    );
    // Copy the file to the destination path
    std::fs::copy(&data.path, &dest_file_path)?;

    Ok(CopiedData {
        input: data.path,
        output: dest_file_path,
    })
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
    use crate::config::Paths;
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
            log: Default::default(),
            other: Default::default(),
        };
        config.create_dirs().unwrap();
        config
    }

    #[test]
    fn test_copy_dicom_data_success() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config = get_test_config(&temp_dir);

        let patient_id = "12345";
        let date_of_birth = "19850615";
        let input_file = config.paths.input_dir.join("test.dcm");

        create_test_dicom_file(patient_id, date_of_birth, &input_file)
            .expect("Unable to create test DICOM file.");

        let sorting_data = DicomData {
            sop_instance_uid: "9.3.12.2.1107.5.1.7.130037.30000025021708505036500000024"
                .to_string(),
            modality: "CT".to_string(),
            patient_id: patient_id.to_string(),
            date_of_birth: date_of_birth.to_string(),
            path: input_file.clone(),
        };

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
    fn test_copy_dicom_data_invalid_date_of_birth_format() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config = get_test_config(&temp_dir);
        let patient_id = "12345";
        let invalid_date_of_birth = "1985";
        let input_file = config.paths.input_dir.join("test.dcm");

        create_test_dicom_file(patient_id, invalid_date_of_birth, &input_file)
            .expect("Unable to create test DICOM file.");

        let sorting_data = DicomData {
            sop_instance_uid: "9.3.12.2.1107.5.1.7.130037.30000025021708505036500000024"
                .to_string(),
            modality: "CT".to_string(),
            patient_id: patient_id.to_string(),
            date_of_birth: invalid_date_of_birth.to_string(),
            path: input_file.clone(),
        };

        // Call the function
        let result = copy_dicom_data(sorting_data, &config);

        // Validate that the function returns an error
        assert!(result.is_err());

        // Validate that the file was not copied
        assert!(is_dir_empty(&config.paths.output_dir));
        std::fs::remove_dir_all(config.paths.input_dir.parent().unwrap()).unwrap()
    }

    #[test]
    fn test_copy_dicom_data_missing_source_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config = get_test_config(&temp_dir);
        let patient_id = "12345";
        let date_of_birth = "19850615";
        let input_file = config.paths.input_dir.join("test.dcm");

        let sorting_data = DicomData {
            sop_instance_uid: "9.3.12.2.1107.5.1.7.130037.30000025021708505036500000024"
                .to_string(),
            modality: "CT".to_string(),
            patient_id: patient_id.to_string(),
            date_of_birth: date_of_birth.to_string(),
            path: input_file.clone(),
        };

        // Call the function
        let result = copy_dicom_data(sorting_data, &config);

        // Validate that the function returns an error
        assert!(result.is_err());
    }
}
