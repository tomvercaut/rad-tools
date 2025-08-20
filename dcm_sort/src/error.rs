use dicom_core::Tag;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("Patient ID is undefined or not set.")]
    PatientIdUnknown,
    #[error("DICOM instance requires a Patient ID.")]
    DicomInstanceMissingPatientId,
    #[error("DICOM instance requires a SOP Instance UID.")]
    DicomInstanceMissingSopInstanceUid,
    #[error("DICOM instance requires a Study Instance UID.")]
    DicomInstanceMissingStudyInstanceUid,
    #[error("DICOM instance requires a Series Instance UID.")]
    DicomInstanceMissingSeriesInstanceUid,
    #[error("DICOM instance requires a Modality.")]
    DicomInstanceMissingModality,
    #[error("Unable to access tag {0} in DICOM object.")]
    DicomElementAccessError(Tag),
    #[error("Unable to convert DICOM tag [{0}] value to string")]
    DicomElementStringConvertValue(Tag),
}

pub type Result<T> = std::result::Result<T, Error>;
