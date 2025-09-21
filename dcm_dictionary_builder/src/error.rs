use dicom_core::header::ParseTagError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to download data")]
    DownloadFailed(#[from] ureq::Error),
    #[error("IO error")]
    IO(#[from] std::io::Error),
    #[error("Failed to parse DICOM tag")]   
    ParseTagFailed(#[from] ParseTagError),
    #[error("Failed to parse DICOM VR")]
    ParseVrFailed,
    #[error("Failed to parse DICOM VM")]
    ParseVmFailed,
    #[error("Failed to parse DICOM version")]
    ParseVersionFailed,
    #[error("Failed to parse dictionary DICOM tag line")]   
    DictionaryTagLineFormatInvalid
}

pub type Result<T> = std::result::Result<T, Error>;