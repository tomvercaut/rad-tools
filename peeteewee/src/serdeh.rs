use crate::decode::*;
use crate::encode::*;
use serde::Deserialize;

pub trait SerdeBase64<T>
where
    T: Base64Encoder,
{
    fn deserialize<'de, D>(deserializer: D) -> Result<T, D::Error>
    where
        D: serde::Deserializer<'de>;
    fn serialize<S>(t: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}

macro_rules! impl_serde_base64_vec {
    ($t:ty, $coder_type:ident, $decoder_type:ty) => {
        pub struct $coder_type {}

        impl $coder_type {
            pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<$t>, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de::Error;
                String::deserialize(deserializer).and_then(|s| {
                    <$decoder_type>::decode(s).map_err(|e| Error::custom(e.to_string()))
                })
            }

            pub fn serialize<S>(encoder: &Vec<$t>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let ts = encoder.to_base64();
                serializer.serialize_str(&ts)
            }
        }
    };
}

macro_rules! impl_serde_base64_internal_cast_vec {
    ($SerializedType:ty,$InternalType:ty, $coder_type:ident, $decoder_type:ty) => {
        pub struct $coder_type {}

        impl $coder_type {
            pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<$InternalType>, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de::Error;

                String::deserialize(deserializer)
                    .and_then(|s| {
                        <$decoder_type>::decode(s).map_err(|e| Error::custom(e.to_string()))
                    })
                    .map(|v| {
                        v.iter()
                            .map(|x| *x as $InternalType)
                            .collect::<Vec<$InternalType>>()
                    })
            }

            pub fn serialize<S>(encoder: &[$InternalType], serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let t = encoder
                    .iter()
                    .map(|x| *x as $SerializedType)
                    .collect::<Vec<$SerializedType>>();
                let ts = t.to_base64();
                serializer.serialize_str(&ts)
            }
        }
    };
}

impl_serde_base64_vec!(u16, SerdeBase64U16, Base64DecoderVecU16);
impl_serde_base64_vec!(i16, SerdeBase64I16, Base64DecoderVecI16);
impl_serde_base64_vec!(u32, SerdeBase64U32, Base64DecoderVecU32);
impl_serde_base64_vec!(i32, SerdeBase64I32, Base64DecoderVecI32);
impl_serde_base64_vec!(u64, SerdeBase64U64, Base64DecoderVecU64);
impl_serde_base64_vec!(i64, SerdeBase64I64, Base64DecoderVecI64);
impl_serde_base64_vec!(u128, SerdeBase64U128, Base64DecoderVecU128);
impl_serde_base64_vec!(i128, SerdeBase64I128, Base64DecoderVecI128);
impl_serde_base64_vec!(f32, SerdeBase64F32, Base64DecoderVecF32);
impl_serde_base64_vec!(f64, SerdeBase64F64, Base64DecoderVecF64);

impl_serde_base64_internal_cast_vec!(f32, f64, SerdeBase64F32WithInternalF64, Base64DecoderVecF32);

pub fn serde_deserialize_base64_f32_as_f64<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)
        .and_then(|s| Base64DecoderVecF32::decode(s).map_err(|e| Error::custom(e.to_string())))
        .map(|v| v.iter().map(|x| *x as f64).collect::<Vec<f64>>())
}

pub fn serde_deserialize_base64_to_u64s<'de, D>(deserializer: D) -> Result<Vec<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)
        .and_then(|s| Base64DecoderVecU64::decode(s).map_err(|e| Error::custom(e.to_string())))
}
