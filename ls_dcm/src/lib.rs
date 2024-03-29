use dicom_core::value::ConvertValueError;

pub mod io;
pub mod model;

#[derive(thiserror::Error, Debug)]
pub enum DicomError {
    #[error("IO error")]
    IO(#[from] std::io::Error),
    #[error("Error while reading a DICOM file")]
    DicomObjectError(#[from] dicom_object::ReadError),
    #[error("Error while accessing a DICOM element a DICOM file")]
    DicomAccessError(#[from] dicom_object::AccessError),
    #[error("Error while convert DICOM value")]
    DicomConvertValueError(#[from] ConvertValueError),
    #[error("Unsupported SOP Class UID for reading: {0}")]
    UnsupportedSOPClassUIDReader(String),
}
