mod ct;
mod common;
mod utils;

use dicom_core::Tag;
pub use ct::*;
use dicom_core::value::ConvertValueError;
pub(crate) use utils::*;
use crate::PatientPositionError;

#[derive(thiserror::Error, Debug)]
pub enum DcmIOError {
    #[error("Failed to read DICOM file")]
    DicomReadError(#[from] dicom_object::ReadError),
    #[error("Unable to find DICOM element")]
    DicomElementAccessError(#[from] dicom_object::AccessError),
    #[error("Unable to convert value from DICOM element")]
    ConvertValueError(#[from] ConvertValueError),
    #[error("Unable to parse date/time")]
    ChronoError(#[from] chrono::ParseError),
    #[error("Invalid date range: {0:#?}")]
    InvalidDateRange(dicom_core::value::range::Error),
    #[error("Invalid time")]
    InvalidTime,
    #[error("Invalid date/time")]
    InvalidDateTime,
    #[error("Unable to create RotationDirection from DICOM element")]
    InvalidRotationDirection(#[from] crate::RotationDirectionError),
    #[error("Unable to create PhotometricInterpration from DICOM element")]
    InvalidPhotometricInterpretation(#[from] crate::PhotometricInterpretationError),
    #[error("Unable to create Pixel Representation from DICOM element")]
    InvalidPixelRepresentation(#[from] crate::PixelRepresentationError),
    #[error("Unable to create Rescale Type from DICOM element")]
    InvalidRescaleType(#[from] crate::RescaleTypeError),
    #[error("Unable to create PatientPosition from DICOM element")]
    PatientPositionError(#[from] PatientPositionError),
    #[error("Invalid input data for FocalSpots: {0:#?}")]
    InvalidFocalSpots(Vec<f64>),
    #[error("Invalid input data for Data Collection Center Patient: {0:#?}")]
    InvalidDataCollectionCenterPatient(Vec<f64>),
    #[error("Invalid input data for Reconstruction Target Center Patient: {0:#?}")]
    InvalidReconstructionTargetCenterPatient(Vec<f64>),
    #[error("Element read with tag [{0:#?}] is not a sequence")]
    ElementIsNotSequence(Tag),
    #[error("Element read with tag [{0:#?}] is not a valid Image Position Patient")]
    InvalidImagePositionPatient(Vec<f64>),
    #[error("Element read with tag [{0:#?}] is not a valid Image Orientation Patient")]
    InvalidImageOrientationPatient(Vec<f64>),
    #[error("Element read with tag [{0:#?}] is a not supported Pixel Sequence")]
    PixelSequenceNotSupported(Tag),
    #[error("Unable to create Pixel Spacing from DICOM element")]
    InvalidPixelSpacing(Vec<f64>),
}
