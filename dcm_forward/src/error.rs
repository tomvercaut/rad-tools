#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Required executable not found in PATH")]
    CommonSystemError(#[from] rad_tools_common::system::Error),
    #[error("Unable to kill a previously initiated DICOM listener")]
    UnableToKillDicomListener,
    #[error("Unable to start a DICOM listener")]
    UnableToStartDicomListener,
    #[error("DCMTK echoscu can't be started due to a commandline syntax error")]
    DcmtkEchoCommandlineSyntaxError,
    #[error("DCMTK echoscu: cannot initialize network")]
    DcmtkEchoCannotInitNetwork,
    #[error("DCMTK echoscu: association aborted")]
    DcmtkEchoAssociationAborted,
    #[error("DCMTK echoscu: other error")]
    DcmtkEchoOtherError,
}

pub type Result<T> = std::result::Result<T, Error>;
