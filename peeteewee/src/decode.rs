use crate::PeeTeeWeeError;
use crate::PeeTeeWeeError::{
    InvalidByteSequence, InvalidBytesF32, InvalidBytesF64, InvalidBytesU16,
};
use base64::Engine;

pub trait Base64Decoder<T, R> {
    /// Convert a base64-encoded string into R.
    ///
    /// # Arguments
    ///
    /// * `s` - A base64-encoded string.
    ///
    /// # Returns
    ///
    /// * `Ok(R)` - Type R if the conversion is successful.
    ///
    /// * `Err(DosimetryToolsError)` - An error if the conversion fails.
    fn decode<S: AsRef<str>>(s: S) -> Result<R, PeeTeeWeeError>;
}

macro_rules! impl_base64_decoder_vector {
    ($t:ty, $decoder_type:ident) => {
        pub struct $decoder_type {}

        impl Base64Decoder<$t, Vec<$t>> for $decoder_type {
            /// Convert a base64-encoded string into Vec<$t>.
            ///
            /// # Arguments
            ///
            /// * `s` - A base64-encoded string.
            ///
            /// # Returns
            ///
            /// * `Ok(Vec<$t>)` - Vec<$t> if the conversion is successful.
            ///
            /// * `Err(DosimetryToolsError)` - An error if the conversion fails.
            fn decode<S: AsRef<str>>(s: S) -> Result<Vec<$t>, PeeTeeWeeError> {
                let bytes = base64::engine::general_purpose::STANDARD.decode(s.as_ref())?;
                let n = bytes.len();
                const N_BYTES: usize = std::mem::size_of::<$t>();
                if n % N_BYTES != 0 {
                    return Err(InvalidByteSequence(n, N_BYTES));
                }
                let mut v = Vec::with_capacity(n / N_BYTES);
                let mut i = 0;
                while i < n {
                    let mut buf: [u8; N_BYTES] = Default::default();
                    for j in 0..N_BYTES {
                        buf[j] = bytes[i + j];
                    }
                    let f = <$t>::from_le_bytes(buf);
                    v.push(f);
                    i += N_BYTES;
                }
                Ok(v)
            }
        }
    };
}

impl_base64_decoder_vector!(u16, Base64DecoderVecU16);
impl_base64_decoder_vector!(i16, Base64DecoderVecI16);
impl_base64_decoder_vector!(u32, Base64DecoderVecU32);
impl_base64_decoder_vector!(i32, Base64DecoderVecI32);
impl_base64_decoder_vector!(u64, Base64DecoderVecU64);
impl_base64_decoder_vector!(i64, Base64DecoderVecI64);
impl_base64_decoder_vector!(u128, Base64DecoderVecU128);
impl_base64_decoder_vector!(i128, Base64DecoderVecI128);
impl_base64_decoder_vector!(f32, Base64DecoderVecF32);
impl_base64_decoder_vector!(f64, Base64DecoderVecF64);
