use base64::{write::EncoderWriter, Engine};

use crate::PeeTeeWeeError;

pub trait ToLeBytes<T, const N: usize> {
    fn to_le_bytes(t: &T) -> [u8; N];
}

pub trait SliceToLeBytes<S, const N: usize> {
    fn to_le_bytes(v: S) -> Vec<u8>;
}

pub trait Base64EncodeSlice<S> {
    fn to_base64(v: S) -> String;
}

pub trait Base64Encoder {
    fn to_base64(&self) -> String;
}

macro_rules! impl_to_le_bytes {
    ($t:ty, $N: literal) => {
        impl ToLeBytes<$t, $N> for $t {
            fn to_le_bytes(t: &$t) -> [u8; $N] {
                t.to_le_bytes()
            }
        }
    };
}

macro_rules! impl_slice_to_le_bytes {
    ($s:ty, $N: literal) => {
        impl<'a> SliceToLeBytes<$s, $N> for $s {
            fn to_le_bytes(v: $s) -> Vec<u8> {
                let n_bytes = v.len() * $N;
                let mut bytes = Vec::with_capacity(n_bytes);
                for d in v {
                    let buf = d.to_le_bytes();
                    bytes.extend_from_slice(&buf);
                }
                bytes
            }
        }
    };
}

macro_rules! impl_base64_encode_slice {
    ($s:ty, $N: literal) => {
        impl<'a> Base64EncodeSlice<$s> for $s
        where
            $s: SliceToLeBytes<$s, $N>,
        {
            fn to_base64(v: $s) -> String {
                let bytes = <$s>::to_le_bytes(v);
                base64::engine::general_purpose::STANDARD.encode(&bytes)
            }
        }
    };
}

macro_rules! impl_base64_encode_vec {
    ($t:ty) => {
        impl Base64Encoder for Vec<$t> {
            /// Encode a vector of `$t` values into a base64-encoded string.
            ///
            /// # Arguments
            ///
            /// * `v` - vector of `$t` values
            ///
            /// Return a base64-encoded string.
            fn to_base64(&self) -> String {
                let slice = self.as_slice();
                <&[$t]>::to_base64(slice)
            }
        }
    };
}

impl_to_le_bytes!(u16, 2);
impl_to_le_bytes!(i16, 2);
impl_to_le_bytes!(u32, 4);
impl_to_le_bytes!(i32, 4);
impl_to_le_bytes!(u64, 8);
impl_to_le_bytes!(i64, 8);
impl_to_le_bytes!(u128, 16);
impl_to_le_bytes!(i128, 16);
impl_to_le_bytes!(f32, 4);
impl_to_le_bytes!(f64, 8);

impl_slice_to_le_bytes!(&'a [u16], 2);
impl_slice_to_le_bytes!(&'a [i16], 2);
impl_slice_to_le_bytes!(&'a [u32], 4);
impl_slice_to_le_bytes!(&'a [i32], 4);
impl_slice_to_le_bytes!(&'a [u64], 8);
impl_slice_to_le_bytes!(&'a [i64], 8);
impl_slice_to_le_bytes!(&'a [u128], 16);
impl_slice_to_le_bytes!(&'a [i128], 16);
impl_slice_to_le_bytes!(&'a [f32], 4);
impl_slice_to_le_bytes!(&'a [f64], 8);

impl_base64_encode_slice!(&'a [u16], 2);
impl_base64_encode_slice!(&'a [i16], 2);
impl_base64_encode_slice!(&'a [u32], 4);
impl_base64_encode_slice!(&'a [i32], 4);
impl_base64_encode_slice!(&'a [u64], 8);
impl_base64_encode_slice!(&'a [i64], 8);
impl_base64_encode_slice!(&'a [u128], 16);
impl_base64_encode_slice!(&'a [i128], 16);
impl_base64_encode_slice!(&'a [f32], 4);
impl_base64_encode_slice!(&'a [f64], 8);

impl_base64_encode_vec!(u16);
impl_base64_encode_vec!(i16);
impl_base64_encode_vec!(u32);
impl_base64_encode_vec!(i32);
impl_base64_encode_vec!(u64);
impl_base64_encode_vec!(i64);
impl_base64_encode_vec!(u128);
impl_base64_encode_vec!(i128);
impl_base64_encode_vec!(f32);
impl_base64_encode_vec!(f64);

#[cfg(test)]
mod test {
    use crate::decode::{Base64Decoder, Base64DecoderVecU16, Base64DecoderVecU64};

    use super::*;

    #[test]
    fn decoder_encoder_u16s() {
        let mut s = "AQABDA==";
        let mut v = Base64DecoderVecU16::decode(s).unwrap();
        let mut t = v.to_base64();
        assert_eq!(s, t);
        s = "AbCdEfGh";
        v = Base64DecoderVecU16::decode(s).unwrap();
        t = v.to_base64();
        assert_eq!(s, t);
    }

    #[test]
    fn encode_decode_u16_vec() {
        let e = vec![2u16, 3u16, 4u16];
        let encoded = e.to_base64();
        let v = Base64DecoderVecU16::decode(encoded).unwrap();
        assert_eq!(e, v);
    }

    #[test]
    fn encode_decode_u64_vec() {
        let e = vec![2u64, 3, 4];
        let encoded = e.to_base64();
        let v = Base64DecoderVecU64::decode(encoded).unwrap();
        assert_eq!(e, v);
    }
}
