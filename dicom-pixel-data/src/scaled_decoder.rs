use crate::{DecodingError, PixelDecoder, PixelRepresentation, PixelValues, PlanarConfiguration};
use crate::raw_decoder::{DecodingData, RawPixelDecoder};

#[derive(Clone, Debug, Default)]
pub struct ScaledPixelDecoder {
    internal: RawPixelDecoder,
}

impl<'a> PixelDecoder<DecodingData<'a>> for ScaledPixelDecoder {
    fn decode(&self, params: &DecodingData<'a>) -> Result<PixelValues, DecodingError> {
        let raw = self.internal.decode(params)?;
        match raw {
            PixelValues::U8(raw) => {
                Ok(PixelValues::F64(scale_u8(raw, params)))
            }
            PixelValues::U16(raw) => { Ok(PixelValues::F64(scale_u16(raw, params))) }
            PixelValues::U32(raw) => { Ok(PixelValues::F64(scale_u32(raw, params))) }
            PixelValues::U64(raw) => { Ok(PixelValues::F64(scale_u64(raw, params))) }
            PixelValues::I8(raw) => { Ok(PixelValues::F64(scale_i8(raw, params))) }
            PixelValues::I16(raw) => { Ok(PixelValues::F64(scale_i16(raw, params))) }
            PixelValues::I32(raw) => { Ok(PixelValues::F64(scale_i32(raw, params))) }
            PixelValues::I64(raw) => { Ok(PixelValues::F64(scale_i64(raw, params))) }
            PixelValues::F32(raw) => { Ok(PixelValues::F64(scale_f32(raw, params))) }
            PixelValues::F64(raw) => { Ok(PixelValues::F64(scale_f64(raw, params))) }
        }
    }
}

macro_rules! scale_pixel {
    ($func:ident, $t:ty) => {
        fn $func(raw: Vec<$t>, params: &DecodingData) -> Vec<f64> {
            let n = raw.len();
            if n == 0 {
                return vec![];
            }
            let mut v = Vec::with_capacity(n);
            for value in &raw {
                v.push(*value as f64 * params.rescale_slope + params.rescale_intercept);
            }
            v
        }
    };
}

scale_pixel!(scale_u8, u8);
scale_pixel!(scale_u16, u16);
scale_pixel!(scale_u32, u32);
scale_pixel!(scale_u64, u64);
scale_pixel!(scale_i8, i8);
scale_pixel!(scale_i16, i16);
scale_pixel!(scale_i32, i32);
scale_pixel!(scale_i64, i64);
scale_pixel!(scale_f32, f32);
scale_pixel!(scale_f64, f64);


#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use dicom_dictionary_std::uids::IMPLICIT_VR_LITTLE_ENDIAN;
    use crate::PhotometricInterpretation;

    use super::*;

    #[test]
    fn scale_monochrome_u16bit() {
        let pixel_data = vec![
            0u8, 0,
            0x1e, 0x1c,
            0x1f, 0x1c,
            0x1a, 0x1c,
            0x1c, 0x1c,
            0x1c, 0x1c,
            0x1d, 0x1c,
            0x1b, 0x1c,
        ];
        let mock_data = DecodingData {
            transfer_syntax: IMPLICIT_VR_LITTLE_ENDIAN.to_string(),
            data: Cow::Borrowed(&pixel_data),
            rows: 2,
            columns: 4,
            number_of_frames: 1,
            photometric_interpretation: PhotometricInterpretation::Monochrome2,
            samples_per_pixel: 1,
            planar_configuration: PlanarConfiguration::ColorByPixel,
            bits_allocated: 16,
            bits_stored: 16,
            high_bit: 15,
            pixel_represenation: PixelRepresentation::UnsignedInteger,
            rescale_intercept: -8192.0,
            rescale_slope: 2.0,
        };
        
        let decoder = ScaledPixelDecoder::default();

        match decoder.decode(&mock_data) {
            Ok(pixel_values) => {
                match pixel_values {
                    PixelValues::F64(pixels) => {
                        let expected = [
                            mock_data.rescale_slope * (0u16 as f64) + mock_data.rescale_intercept,
                            mock_data.rescale_slope * (0x1c1e as f64) + mock_data.rescale_intercept,
                            mock_data.rescale_slope * (0x1c1f as f64) + mock_data.rescale_intercept,
                            mock_data.rescale_slope * (0x1c1a as f64) + mock_data.rescale_intercept,
                            mock_data.rescale_slope * (0x1c1c as f64) + mock_data.rescale_intercept,
                            mock_data.rescale_slope * (0x1c1c as f64) + mock_data.rescale_intercept,
                            mock_data.rescale_slope * (0x1c1d as f64) + mock_data.rescale_intercept,
                            mock_data.rescale_slope * (0x1c1b as f64) + mock_data.rescale_intercept
                        ];
                        assert_eq!(&expected, pixels.as_slice());
                    }
                    _ => {
                        panic!("Failed to decode the data: invalid pixel data type");
                    }
                }
            }
            Err(e) => {
                panic!("Failed to decode the data: {:#?}", e);
            }
        }
    }
}