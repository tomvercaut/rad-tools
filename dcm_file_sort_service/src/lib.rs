mod cli;
mod config;

pub use cli::Cli;
pub use config::Config;

use crate::Error::InvalidDateOfBirth;
use dicom_object::{open_file, ReadError};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, trace};
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
}

pub type Result<T> = std::result::Result<T, Error>;

/// Data used to sort the DICOM file
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub(crate) struct SortingData {
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

///
/// Represents the movement of a file from an input path to an output path.
///
/// This struct is used to log and track the relocation of files during processing.
///
/// # Fields
/// * `input` - The original file path where the DICOM file was located.
/// * `output` - The destination file path where the DICOM file was moved.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub(crate) struct MovedData {
    /// Input file path
    pub input: PathBuf,
    /// Output file path
    pub output: PathBuf,
}

impl std::fmt::Display for MovedData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Moved {} to {}",
            self.input.display(),
            self.output.display()
        )
    }
}

pub fn run_service<P: AsRef<Path>>(
    input_dir: P,
    output_dir: P,
    state: Arc<RwLock<ServiceState>>,
    wait_millisecs: u64,
) -> Result<()> {
    'outer: loop {
        let to_sort = get_sorting_data(&input_dir, state.clone())?;
        // if to_sort.is_empty() {
        //     std::thread::sleep(std::time::Duration::from_millis(wait_millisecs));
        //     if should_stop(&state) {
        //         break 'outer;
        //     }
        //     continue;
        // }
        for sd in to_sort {
            if should_stop(&state) {
                break 'outer;
            }
            match copy_dicom_data(sd, &output_dir) {
                Ok(moved_data) => {
                    info!(
                        "Moved {} to {}",
                        moved_data.input.display(),
                        moved_data.output.display()
                    );
                    std::fs::remove_file(moved_data.input)?;
                }
                Err(e) => {
                    error!("Failed to move file: {}", e);
                }
            }
        }
        remove_empty_sub_dirs(&input_dir)?;
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
/// Iterates over the files in the given directory and gathers sorting data for DICOM files.
///
/// This function traverses the specified directory and its subdirectories, examining each
/// file to determine if it is a DICOM file. If so, it attempts to extract metadata and collects
/// it as `SortingData`. The function monitors the shared `state` to ensure it halts processing if
/// the service state is no longer `ServiceState::Running`.
///
/// # Arguments
/// * `input_dir` - The directory to scan for DICOM files.
/// * `state` - A shared atomic state indicating whether the service should continue running.
///
/// # Returns
/// * `Ok(Vec<SortingData>)` - A vector containing `SortingData` for identified DICOM files.
/// * `Err` - If an error occurs during the directory traversal or file processing.
///
/// # Behavior
/// * The function stops processing if the service changes to a non-running state.
/// * Errors during metadata extraction for individual files are logged but do not stop execution.
///
/// # Errors
/// * Returns an error if a directory traversal issue occurs or if it encounters a critical failure
///   while processing files.
fn get_sorting_data<P: AsRef<Path>>(
    input_dir: P,
    state: Arc<RwLock<ServiceState>>,
) -> Result<Vec<SortingData>> {
    let mut v = vec![];
    // for entry in WalkDir::new(&input_dir).into_iter().filter_map(|r| r.ok()) {
    for entry in WalkDir::new(&input_dir) {
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
                    Ok(Some(sorting_data)) => {
                        v.push(sorting_data);
                    }
                    Ok(None) => {}
                    Err(e) => {
                        // It might not be a DICOM file
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
    Ok(v)
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
fn extract_dicom_metadata<P: AsRef<Path>>(file_path: P) -> Result<Option<SortingData>> {
    let dicom_file = open_file(file_path.as_ref())?;

    let sop_instance_uid = dicom_file
        .element_by_name("SOPInstanceUID")
        .ok()
        .and_then(|elem| match elem.string() {
            Ok(s) => Some(s.trim().to_string()),
            Err(e) => {
                error!("Failed to extract SOP Instance UID: {}", e);
                None
            }
        });

    let modality = dicom_file
        .element_by_name("Modality")
        .ok()
        .and_then(|elem| match elem.string() {
            Ok(s) => Some(s.trim().to_string()),
            Err(e) => {
                error!("Failed to extract Modality: {}", e);
                None
            }
        });

    // Extracting Patient ID (Tag: (0010,0020)) and Date of Birth (Tag: (0010,0030)).
    let patient_id = dicom_file
        .element_by_name("PatientID")
        .ok()
        .and_then(|elem| match elem.string() {
            Ok(s) => Some(s.trim().to_string()),
            Err(e) => {
                error!("Failed to extract Patient ID: {}", e);
                None
            }
        });

    let date_of_birth = dicom_file
        .element_by_name("PatientBirthDate")
        .ok()
        .and_then(|elem| match elem.string() {
            Ok(s) => Some(s.trim().to_string()),
            Err(e) => {
                error!("Failed to extract Date of Birth: {}.\nTrying to extract part of the patient ID.", e);
                if let Some(pid) = patient_id.as_ref() {
                    let t = format!("00{}", &pid[0..6]);
                    return Some(t);
                }
                None
            }
        });

    if let (Some(sop_instance_uid), Some(modality), Some(pid), Some(dob)) =
        (sop_instance_uid, modality, patient_id, date_of_birth)
    {
        Ok(Some(SortingData {
            sop_instance_uid,
            modality,
            patient_id: pid,
            date_of_birth: dob,
            path: file_path.as_ref().to_path_buf(),
        }))
    } else {
        Ok(None)
    }
}

///
/// Copies a DICOM file to a designated output directory based on its metadata.
///
/// # Arguments
/// * `data` - A `SortingData` struct containing metadata (e.g., Patient ID and Date of Birth) and the original file path.
/// * `output_dir` - The path to the output directory where the file should be copied.
///
/// # Returns
/// * `Ok(MovedData)` - If the file is successfully copied to the output directory.
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
fn copy_dicom_data<P: AsRef<Path>>(data: SortingData, output_dir: P) -> Result<MovedData> {
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
        output_dir
            .as_ref()
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
        output_path.join(format!("{}{}.dcm", &data.modality, &data.sop_instance_uid));

    debug!(
        "Copying file: {} -> {}",
        source_path.display(),
        dest_file_path.display()
    );
    // Copy the file to the destination path
    std::fs::copy(source_path, &dest_file_path)?;

    Ok(MovedData {
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
    use dicom_core::{DataElement, PrimitiveValue};
    use dicom_dictionary_std::tags;
    use dicom_dictionary_std::uids::CT_IMAGE_STORAGE;
    use dicom_object::{FileMetaTableBuilder, InMemDicomObject};

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

    #[test]
    fn test_copy_dicom_data_success() {
        let temp_dir = tempfile::tempdir().unwrap();
        let input_dir = temp_dir.path().join("input");
        let output_dir = temp_dir.path().join("output");
        std::fs::create_dir_all(&input_dir).unwrap();
        std::fs::create_dir_all(&output_dir).unwrap();
        let patient_id = "12345";
        let date_of_birth = "19850615";
        let input_file = input_dir.join("test.dcm");

        let r = create_test_dicom_file(patient_id, date_of_birth, &input_file);
        if r.as_ref().is_err() {
            panic!("Failed to create test file: {:?}", r);
        }
        assert!(r.is_ok());

        let sorting_data = SortingData {
            sop_instance_uid: "9.3.12.2.1107.5.1.7.130037.30000025021708505036500000024"
                .to_string(),
            modality: "CT".to_string(),
            patient_id: patient_id.to_string(),
            date_of_birth: date_of_birth.to_string(),
            path: input_file.clone(),
        };

        let output_file_name = format!(
            "{}{}.dcm",
            &sorting_data.modality, &sorting_data.sop_instance_uid
        );

        // Call the function
        let moved_data = copy_dicom_data(sorting_data, &output_dir).unwrap();

        // Validate that the file was copied to the correct output directory
        let expected_output_path = output_dir
            .join(&date_of_birth[4..])
            .join(patient_id)
            .join(output_file_name);
        debug!("Expected output path: {}", expected_output_path.display());
        debug!("Actual output path: {}", moved_data.output.display());
        assert!(moved_data.output.exists());
        // assert!(expected_output_path.exists());

        // Validate the channel message
        assert_eq!(moved_data.input, input_file);
        assert_eq!(moved_data.output, expected_output_path);
        std::fs::remove_dir_all(temp_dir).unwrap()
    }

    #[test]
    fn test_copy_dicom_data_invalid_date_of_birth_format() {
        let temp_dir = tempfile::tempdir().unwrap();
        let input_dir = temp_dir.path().join("input");
        let output_dir = temp_dir.path().join("output");
        std::fs::create_dir_all(&input_dir).unwrap();
        std::fs::create_dir_all(&output_dir).unwrap();
        let patient_id = "12345";
        let invalid_date_of_birth = "1985";
        let input_file = input_dir.join("test.dcm");

        let sorting_data = SortingData {
            sop_instance_uid: "9.3.12.2.1107.5.1.7.130037.30000025021708505036500000024"
                .to_string(),
            modality: "CT".to_string(),
            patient_id: patient_id.to_string(),
            date_of_birth: invalid_date_of_birth.to_string(),
            path: input_file.clone(),
        };

        // Call the function
        let result = copy_dicom_data(sorting_data, &output_dir);

        // Validate that the function returns an error
        assert!(result.is_err());

        // Validate that the file was not copied
        assert!(is_dir_empty(&output_dir));
        std::fs::remove_dir_all(temp_dir).unwrap()
    }

    #[test]
    fn test_copy_dicom_data_missing_source_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let input_dir = temp_dir.path().join("input");
        let output_dir = temp_dir.path().join("output");
        std::fs::create_dir_all(&input_dir).unwrap();
        std::fs::create_dir_all(&output_dir).unwrap();
        let patient_id = "12345";
        let date_of_birth = "19850615";
        let input_file = input_dir.join("test.dcm");

        let sorting_data = SortingData {
            sop_instance_uid: "9.3.12.2.1107.5.1.7.130037.30000025021708505036500000024"
                .to_string(),
            modality: "CT".to_string(),
            patient_id: patient_id.to_string(),
            date_of_birth: date_of_birth.to_string(),
            path: input_file.clone(),
        };

        // Call the function
        let result = copy_dicom_data(sorting_data, &output_dir);

        // Validate that the function returns an error
        assert!(result.is_err());
    }
}
