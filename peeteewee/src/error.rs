use std::num::{ParseFloatError, ParseIntError};
use std::str::Utf8Error;

#[derive(thiserror::Error, Debug)]
pub enum PeeTeeWeeError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Serde deserialization error: {0}")]
    SerdeDeserializeError(String),
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
    #[error(transparent)]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("Invalid number of bytes [{0}] for a datatype sequence with {1} byte size.")]
    InvalidByteSequence(usize, usize),
    #[error("Invalid number of bytes [{0}] to convert data to a u16.")]
    InvalidBytesU16(usize),
    #[error("Invalid number of bytes [{0}] to convert data to a 32 bit float.")]
    InvalidBytesF32(usize),
    #[error("Invalid number of bytes [{0}] to convert data to a 64 bit float.")]
    InvalidBytesF64(usize),
    #[error("Unable to convert string [{0}] to TaskType")]
    ParseTaskTypeError(String),
    #[error("Unable to convert string [{0}] to DetectorType")]
    ParseDetectorTypeError(String),
    #[error("Unable to create boolean from {0}")]
    ParseBoolError(String),
    #[error("Unable to create RotationDirection from {0}")]
    InvalidStrToRotationDirection(String),
    #[error("Unable to create Orientation from {0}")]
    InvalidStrToOrientation(String),
    #[error("Unable to create CurveType from {0}")]
    InvalidStrToCurveType(String),
    #[error("Unable to get String value for key: {0}")]
    KeyValueString(String),
    #[error("Unable to create Octavius1500 from Xcc: {0}")]
    Octavius1500FromXccError(String),
    #[error("Index is out of bound.")]
    IndexOutOfBound,
}
