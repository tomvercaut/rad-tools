#[derive(thiserror::Error, Debug, PartialEq)]
pub enum DecodeError {
    #[error("Unsupported transfer syntax in decoder: {0}")]
    UnsupportedTransferSyntax(String),
    #[error("Expected byte length: {0}, actual byte length: {1}")]
    ExpectedByteMismatch(usize, usize),
    #[error("Unsupported photometric interpretation: {0}")]
    UnsupportedPhotometricInterpretation(String),
    #[error("Unsupported number of bits to decode per pixel: {0}")]
    UnsupportedNumberOfBits(usize),
    #[error("High bit for a u8 mask is out of bound: {0}")]
    U8MaskHighBitOutOfBound(u8),
    #[error("High bit for a u16 mask is out of bound: {0}")]
    U16MaskHighBitOutOfBound(u16),
}

pub enum PixelValues {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
    I8(Vec<i8>),
    I16(Vec<i16>),
    I32(Vec<i32>),
    F32(Vec<f32>),
    F64(Vec<f64>),
}

/// Trait for decoding pixel values based on given a set of decoding parameters.
///
/// The `PixelDecoder` trait provides a common interface for decoding pixel values.
/// Implementations of this trait should implement the `decode` method, which takes
/// a reference to `DecodingParameters` and returns a `Result` containing the
/// decoded `PixelValues` or an error of type `DecodeError`.
pub trait PixelDecoder<DecodingParameters> {
    
    /// Decodes the given parameters to obtain pixel values.
///
/// # Arguments
///
/// * `params` - A reference to the `DecodingParameters` struct containing the necessary
///              parameters for decoding.
///
/// # Returns
///
/// Returns a `Result` containing the decoded `PixelValues` on success, or a `DecodeError` on failure.
fn decode(&self, params: &DecodingParameters) -> Result<PixelValues, DecodeError>;
}

