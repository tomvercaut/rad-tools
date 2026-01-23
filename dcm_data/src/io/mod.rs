mod common;
mod ct;
mod rtdose;
mod rtplan;
mod rtstruct;
mod utils;

use crate::{
    ContourGeometryError, FluenceModeError, PatientPositionError, RTBeamLimitingDeviceTypeError,
    RadiationTypeError, TreatmentDeliveryTypeError,
};
pub use ct::read_ct_image;
use dicom_core::Tag;
use dicom_core::value::{CastValueError, ConvertValueError};
pub use rtdose::{obj_to_rtdose, read_rtdose};
pub use rtplan::{obj_to_rtplan, read_rtplan};
pub use rtstruct::{obj_to_rtstruct, read_rtstruct};
pub(crate) use utils::*;

#[derive(thiserror::Error, Debug)]
pub enum DcmIOError {
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
    #[error("Invalid decoded pixel data: {0:#?}")]
    InvalidDecodedPixelData(#[from] dicom_pixeldata::Error),
    #[error("Invalid date time, unkown date with a defined time: {0:#?}")]
    InvalidDateTimeEmpytDate(String),
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
    #[error("Unable to create Modality from DICOM element")]
    InvalidModality(#[from] crate::ModalityError),
    #[error("Unable to create Rescale Type from DICOM element")]
    InvalidRescaleType(#[from] crate::RescaleTypeError),
    #[error("Unable to create PatientPosition from DICOM element")]
    PatientPositionError(#[from] PatientPositionError),
    #[error("Unable to create ContourGeometry from DICOM element")]
    ContourGeometryError(#[from] ContourGeometryError),
    #[error("Unable to create RadiationType from DICOM element")]
    RadiationTypeError(#[from] RadiationTypeError),
    #[error("Unable to create TreatmentDeliveryType from DICOM element")]
    TreatmentDeliveryTypeError(#[from] TreatmentDeliveryTypeError),
    #[error("Unable to create FluenceMode from DICOM element")]
    FluenceModeError(#[from] FluenceModeError),
    #[error("Unable to create BeamLimitingDeviceType from DICOM element")]
    BeamLimitingDeviceTypeError(#[from] RTBeamLimitingDeviceTypeError),
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
    #[error("Reader doesn't match with SOP class UID: [{0:#?}]")]
    NoMatchingSopClassUID(String),
    #[error("Invalid RGB string: \"{0:#?}\"")]
    InvalidRGBString(String),
    #[error("Invalid isocenter: [{0:#?}]")]
    InvalidIsocenter(Vec<f64>),
    #[error("Expected VRs ({0:#?} <-> {0:#?}) to match")]
    InvalidVRMatch(dicom_core::VR, dicom_core::VR),
    #[error("Invalid number of tag values: Expected {0}, got {1}")]
    InvalidNumberOfTagValues(usize, usize),
}
