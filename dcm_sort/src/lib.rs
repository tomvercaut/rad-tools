mod error;
mod support;

pub use error::{Error, Result};

use std::path::{Path, PathBuf};

use dicom_dictionary_std::tags::{
    MODALITY, PATIENT_ID, PIXEL_DATA, SERIES_DESCRIPTION, SERIES_INSTANCE_UID, SERIES_NUMBER,
    SOP_INSTANCE_UID, STUDY_DESCRIPTION, STUDY_INSTANCE_UID,
};
use dicom_object::{
    DefaultDicomObject, InMemDicomObject, OpenFileOptions, ReadError,
};
use tracing::{debug, error};

const STUDY_INSTANCE_UID_UNKNOWN: &str = "STUDY_UID_UNKNOWN";
const SERIES_INSTANCE_UID_UNKNOWN: &str = "SERIES_UID_UNKNOWN";
const SERIES_NUMBER_UNKNOWN: &str = "SERIES_NUMBER_UNKNOWN";
const MODALITY_UNKNOWN: &str = "MODALITY_UNKNOWN";

/// Reads a DICOM file and loads its metadata without reading pixel data.
///
/// This function opens a DICOM file and reads all metadata tags up until (but not including)
/// the pixel data element to optimize memory usage and reading speed.
///
/// # Arguments
///
/// * `path` - Path to the DICOM file to read. Can be any type that implements AsRef<Path>
///
/// # Returns
///
/// * `Ok(DefaultDicomObject)` - The DICOM object containing the file's metadata
/// * `Err(ReadError)` - If an error occurs while reading the DICOM file
pub fn read_dicom_file_without_pixels<P>(
    path: P,
) -> std::result::Result<DefaultDicomObject, ReadError>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    debug!("Trying to read DICOM data from: {:#?}", path);
    OpenFileOptions::new()
        .read_until(PIXEL_DATA)
        .open_file(path)
}

/// A trait to convert a DICOM object into another data type.
pub trait TryFromDicomObject: Sized {
    /// Type of error that can be generated while trying to convert a DICOM object.
    type DicomObjectError;

    /// Tries to convert a DICOM object into `Self`.
    ///
    /// If an error is detected, it's returned.
    fn try_from_dicom_obj(
        obj: &InMemDicomObject,
    ) -> std::result::Result<Self, Self::DicomObjectError>;
}

/// Data by which the DICOM files can be categorized.
#[derive(Clone, Debug, PartialEq)]
pub struct Data {
    /// Unique patient identifier
    patient_id: String,
    /// SOP instance UID
    sop_instance_uid: String,
    /// Study Instance UID
    study_uid: String,
    /// Study description
    study_descr: String,
    /// Series Instance UID
    series_uid: String,
    /// Series description
    series_descr: String,
    /// Series number
    series_nr: String,
    /// Modality
    modality: String,
}

impl Data {
    /// Get the patient ID.
    pub fn patient_id(&self) -> &str {
        &self.patient_id
    }

    pub fn sop_instance_uid(&self) -> &str {
        &self.sop_instance_uid
    }

    /// Get the study instance UID.
    pub fn study_uid(&self) -> &str {
        &self.study_uid
    }

    /// Get the study description.
    pub fn study_descr(&self) -> &str {
        &self.study_descr
    }

    /// Get the series instance UID.
    pub fn series_uid(&self) -> &str {
        &self.series_uid
    }

    /// Get the series description.
    pub fn series_descr(&self) -> &str {
        &self.series_descr
    }

    /// Get the series number.
    pub fn series_nr(&self) -> &str {
        &self.series_nr
    }

    /// Get the modality.
    pub fn modality(&self) -> &str {
        &self.modality
    }
}
impl AsRef<Data> for Data {
    fn as_ref(&self) -> &Data {
        self
    }
}

impl TryFromDicomObject for Data {
    type DicomObjectError = Error;

    fn try_from_dicom_obj(
        obj: &InMemDicomObject,
    ) -> std::result::Result<Self, Self::DicomObjectError> {
        let patient_id = match support::get_str(obj, PATIENT_ID) {
            Ok(value) => Ok(value),
            Err(e) => {
                error!("Unable to get Patient ID: {:#?}", e);
                Err(Error::DicomInstanceMissingPatientId)
            }
        }?;
        let sop_instance_uid = match support::get_str(obj, SOP_INSTANCE_UID) {
            Ok(value) => Ok(value),
            Err(e) => {
                error!("Unable to get SOP Instance UID: {:#?}", e);
                Err(Error::DicomInstanceMissingSopInstanceUid)
            }
        }?;
        let study_uid = match support::get_str(obj, STUDY_INSTANCE_UID) {
            Ok(value) => Ok(value),
            Err(e) => {
                error!("Unable to get Study Instance UID: {:#?}", e);
                Err(Error::DicomInstanceMissingStudyInstanceUid)
            }
        }?;
        let study_descr = support::get_str_or_default(obj, STUDY_DESCRIPTION);
        let series_uid = match support::get_str(obj, SERIES_INSTANCE_UID) {
            Ok(value) => Ok(value),
            Err(e) => {
                error!("Unable to get Series Instance UID: {:#?}", e);
                Err(Error::DicomInstanceMissingSeriesInstanceUid)
            }
        }?;
        let series_descr = support::get_str_or_default(obj, SERIES_DESCRIPTION);
        let series_nr = support::get_str_or_default(obj, SERIES_NUMBER);
        let modality = match support::get_str(obj, MODALITY) {
            Ok(value) => Ok(value),
            Err(e) => {
                error!("Unable to get Modality: {:#?}", e);
                Err(Error::DicomInstanceMissingModality)
            }
        }?;
        let data = Data {
            patient_id,
            sop_instance_uid,
            study_uid,
            study_descr,
            series_uid,
            series_descr,
            series_nr,
            modality,
        };
        Ok(data)
    }
}

/// Sanitizes a string by replacing non-allowed characters with underscores ('_').
///
/// The following characters are preserved:
/// - Alphanumeric characters (a-z, A-Z, 0-9)
/// - Space character
/// - Round brackets: '(' and ')'
/// - Square brackets: '[' and ']'
///
/// All other characters are replaced with an underscore.
/// Multiple consecutive underscores and spaces are collapsed into a single one.
/// The returned string is also trimmed.
fn sanitize(input: &str) -> String {
    // Single-pass: normalize disallowed chars to '_' and collapse runs of '_' and ' '.
    let mut out = String::with_capacity(input.len());
    let mut last_should_be_unique = false;

    for c in input.chars() {
        // Normalize: keep allowed chars, otherwise map to '_'
        let n = if is_allowed_char(c) { c } else { '_' };

        // Collapse consecutive '_' or ' '
        let is_collapse_target = n == '_' || n == ' ';
        if !(is_collapse_target && last_should_be_unique) {
            out.push(n);
        } else if is_collapse_target && last_should_be_unique && !out.ends_with('_') {
            let _ = out.pop();
            out.push('_');
        }
        last_should_be_unique = is_collapse_target;
    }

    out.trim_matches(['_', ' ']).to_string()
}

#[inline]
fn is_allowed_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, ' ' | '(' | ')' | '[' | ']')
}

/// Create a file path based on the data.
///
/// Format of the path being created:
/// `<p>/<patient ID>/<study>/<series>/<series nr>/<modality>`
///
/// Where:
/// - `p`: input path
/// - `patient ID`: unique patient identifier
/// - `study`:
///     - study description, if not empty
///     - study instance UID, if not empty
///     - STUDY_UID_UNKNOWN
/// - `series`:
///     - series description, if not empty
///     - series instance UID, if not empty
///     - SERIES_UID_UNKNOWN
/// - `series nr`:
///     - series number, if not empty
///     - SERIES_UID_UNKNOWN
/// - `modality`:
///     - modality, if not empty
///     - MODALITY_UNKNOWN
pub fn to_path_buf<D, P>(d: D, p: P) -> Result<PathBuf>
where
    D: AsRef<Data>,
    P: AsRef<Path>,
{
    let d = d.as_ref();
    // Patient ID must not be empty and is not sanitized to preserve the original value.
    let patient_id = d.patient_id().trim();
    if patient_id.is_empty() {
        return Err(Error::PatientIdUnknown);
    }
    let p = p.as_ref();
    let study = sanitize(if d.study_uid().is_empty() && d.study_descr().is_empty() {
        STUDY_INSTANCE_UID_UNKNOWN
    } else if !d.study_descr().is_empty() {
        d.study_descr()
    } else {
        d.study_uid()
    });
    let series = sanitize(
        if d.series_uid().is_empty() && d.series_descr().is_empty() {
            SERIES_INSTANCE_UID_UNKNOWN
        } else if !d.series_descr().is_empty() {
            d.series_descr()
        } else {
            d.series_uid()
        },
    );
    let serie_number = sanitize(if d.series_nr().is_empty() {
        SERIES_NUMBER_UNKNOWN
    } else {
        d.series_nr()
    });
    let modality = sanitize(if d.modality().is_empty() {
        MODALITY_UNKNOWN
    } else {
        d.modality()
    });
    let pb = p
        .join(patient_id)
        .join(study)
        .join(series)
        .join(serie_number)
        .join(modality);
    Ok(pb)
}

/// Creates a unique file path for a DICOM file within the specified directory.
///
/// This function generates a unique file path by using SOP Instance UID from the DICOM data
/// as the base filename with a ".dcm" extension. If a file with that name already exists,
/// it appends an incrementing number to the filename until a unique path is found.
///
/// # Arguments
///
/// * `data` - The DICOM data containing the SOP Instance UID
/// * `dir` - The directory where the file should be created
///
/// # Returns
///
/// * `Ok(PathBuf)` - A unique path for the DICOM file
/// * `Err(std::io::Error)` - If the directory cannot be created, or if too many files with similar names exist
pub fn unique_dcm_file<P, D>(data: D, dir: P) -> std::io::Result<PathBuf>
where
    P: AsRef<Path>,
    D: AsRef<Data>,
{
    let dir = dir.as_ref();
    let data = data.as_ref();

    let mut output_path = dir.join(format!("{}.dcm", data.sop_instance_uid));
    let mut exists = output_path.exists();

    if !exists {
        return Ok(output_path);
    }

    let mut i = -1isize;
    while exists {
        i += 1;
        output_path = dir.join(format!("{}_{}.dcm", data.sop_instance_uid, i));
        exists = output_path.exists();
        if !exists {
            return Ok(output_path);
        }
        if i == isize::MAX {
            break;
        }
    }
    Err(std::io::Error::other("Too many files with the same name."))
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use tracing::error;

    use crate::{
        Data, MODALITY_UNKNOWN, SERIES_INSTANCE_UID_UNKNOWN, SERIES_NUMBER_UNKNOWN,
        STUDY_INSTANCE_UID_UNKNOWN,
    };

    #[test]
    fn test_sanitize() {
        let test_cases = [
            ("hello world", "hello world"),
            ("hello__world", "hello_world"),
            ("hello  world", "hello_world"),
            ("hello#$%world", "hello_world"),
            ("hello(world)", "hello(world)"),
            ("hello[world]", "hello[world]"),
            ("  hello  world  ", "hello_world"),
            ("hello___world", "hello_world"),
            ("hello_ _world", "hello_world"),
            ("hello   world", "hello_world"),
            ("hello _ world", "hello_world"),
            ("123ABC", "123ABC"),
            ("", ""),
            ("hello@#$%^&*world", "hello_world"),
            ("Test Case(1)[2]", "Test Case(1)[2]"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(super::sanitize(input), expected);
        }
    }

    #[test]
    fn to_path_buf() {
        let datas = [
            Data {
                patient_id: "pt_id".into(),
                sop_instance_uid: "sop_instance_uid_0".into(),
                study_uid: "study".into(),
                study_descr: "study_descr".into(),
                series_uid: "series".into(),
                series_descr: "series_descr".into(),
                series_nr: "1".into(),
                modality: "CT".into(),
            },
            Data {
                patient_id: "pt_id".into(),
                sop_instance_uid: "sop_instance_uid_1".into(),
                study_uid: "study".into(),
                study_descr: "".into(),
                series_uid: "series".into(),
                series_descr: "".into(),
                series_nr: "1".into(),
                modality: "CT".into(),
            },
            Data {
                patient_id: "pt_id".into(),
                sop_instance_uid: "sop_instance_uid_2".into(),
                study_uid: "".into(),
                study_descr: "".into(),
                series_uid: "".into(),
                series_descr: "".into(),
                series_nr: "".into(),
                modality: "".into(),
            },
        ];
        let bufs = [
            PathBuf::new()
                .join(".")
                .join("pt_id")
                .join("study_descr")
                .join("series_descr")
                .join("1")
                .join("CT"),
            PathBuf::new()
                .join(".")
                .join("pt_id")
                .join("study")
                .join("series")
                .join("1")
                .join("CT"),
            PathBuf::new()
                .join(".")
                .join("pt_id")
                .join(STUDY_INSTANCE_UID_UNKNOWN)
                .join(SERIES_INSTANCE_UID_UNKNOWN)
                .join(SERIES_NUMBER_UNKNOWN)
                .join(MODALITY_UNKNOWN),
        ];
        assert_eq!(bufs.len(), datas.len());
        for (data, expected) in datas.iter().zip(bufs.iter()) {
            let r = super::to_path_buf(data, ".");
            if r.is_err() {
                error!("Failed to create PathBuf from input data: {:#?}", data);
            }
            assert!(r.is_ok(), "Failed to create PathBuf from Data.");
            let buf = r.unwrap();
            assert_eq!(expected.to_str().unwrap(), buf.to_str().unwrap());
        }
    }
}
