use dicom_core::value::{CastValueError, ConvertValueError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to read DICOM file")]
    DicomReadError(#[from] dicom_object::ReadError),
    #[error("Unable to find DICOM element")]
    DicomElementAccessError(#[from] dicom_object::AccessError),
    #[error("Unable to convert value from DICOM element")]
    ConvertValueError(#[from] ConvertValueError),
    #[error("Unable to cast internal DICOM value to the requested data type.")]
    CastValueError(#[from] CastValueError),
    #[error("Unable to parse date/time")]
    ChronoError(#[from] chrono::ParseError),
    #[error("Invalid date range: {0:#?}")]
    InvalidDateRange(dicom_core::value::range::Error),
    #[error("Expected VRs ({0:#?} <-> {0:#?}) to match")]
    InvalidVRMatch(dicom_core::VR, dicom_core::VR),
    #[error("Invalid Person Name format: {0:#?}")]
    InvalidPersonNameFormat(String),
    #[error("Invalid number of tag values: Expected {0}, got {1}")]
    InvalidNumberOfTagValues(usize, usize),
}