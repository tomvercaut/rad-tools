mod support;

use std::path::{Path, PathBuf};

use dicom_dictionary_std::tags::{
    MODALITY, PATIENT_ID, SERIES_DESCRIPTION, SERIES_INSTANCE_UID, SERIES_NUMBER,
    STUDY_DESCRIPTION, STUDY_INSTANCE_UID,
};
use dicom_object::InMemDicomObject;

const STUDY_INSTANCE_UID_UNKNOWN: &str = "STUDY_UID_UNKNOWN";
const SERIES_INSTANCE_UID_UNKNOWN: &str = "SERIES_UID_UNKNOWN";
const SERIES_NUMBER_UNKNOWN: &str = "SERIES_NUMBER_UNKNOWN";
const MODALITY_UNKNOWN: &str = "MODALITY_UNKNOWN";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Patient ID is undefined or not set.")]
    PatientIdUnknown,
}

pub type Result<T> = std::result::Result<T, Error>;

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

#[derive(Debug)]
pub enum FromDicomObjectError {}

impl TryFromDicomObject for Data {
    type DicomObjectError = FromDicomObjectError;

    fn try_from_dicom_obj(
        obj: &InMemDicomObject,
    ) -> std::result::Result<Self, Self::DicomObjectError> {
        let patient_id = support::get_str(obj, PATIENT_ID).unwrap();
        let study_uid = support::get_str(obj, STUDY_INSTANCE_UID).unwrap();
        let study_descr = support::get_str_or_default(obj, STUDY_DESCRIPTION);
        let series_uid = support::get_str(obj, SERIES_INSTANCE_UID).unwrap();
        let series_descr = support::get_str_or_default(obj, SERIES_DESCRIPTION);
        let series_nr = support::get_str_or_default(obj, SERIES_NUMBER);
        let modality = support::get_str(obj, MODALITY).unwrap();
        let data = Data {
            patient_id,
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

/// Create a file path based on the data.
///
/// Format of the path being created:
/// <p>/<study>/<series>/<series nr>/<modality>
///
/// where:
/// - `p`: input path
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
pub fn to_path_buf<P>(d: &Data, p: P) -> Result<PathBuf>
where
    P: AsRef<Path>,
{
    if d.patient_id.trim().is_empty() {
        return Err(Error::PatientIdUnknown);
    }
    let p = p.as_ref();
    let pb = p
        .join(d.patient_id())
        .join(if d.study_uid().is_empty() && d.study_descr().is_empty() {
            STUDY_INSTANCE_UID_UNKNOWN
        } else if !d.study_descr().is_empty() {
            d.study_descr()
        } else {
            d.study_uid()
        })
        .join(
            if d.series_uid().is_empty() && d.series_descr().is_empty() {
                SERIES_INSTANCE_UID_UNKNOWN
            } else if !d.series_descr().is_empty() {
                d.series_descr()
            } else {
                d.series_uid()
            },
        )
        .join(if d.series_nr().is_empty() {
            SERIES_NUMBER_UNKNOWN
        } else {
            d.series_nr()
        })
        .join(if d.modality().is_empty() {
            MODALITY_UNKNOWN
        } else {
            d.modality()
        });
    Ok(pb)
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
    fn to_path_buf() {
        let datas = [
            Data {
                patient_id: "pt_id".into(),
                study_uid: "study".into(),
                study_descr: "study_descr".into(),
                series_uid: "series".into(),
                series_descr: "series_descr".into(),
                series_nr: "1".into(),
                modality: "CT".into(),
            },
            Data {
                patient_id: "pt_id".into(),
                study_uid: "study".into(),
                study_descr: "".into(),
                series_uid: "series".into(),
                series_descr: "".into(),
                series_nr: "1".into(),
                modality: "CT".into(),
            },
            Data {
                patient_id: "pt_id".into(),
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
