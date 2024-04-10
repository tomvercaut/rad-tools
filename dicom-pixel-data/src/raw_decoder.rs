use std::borrow::Cow;
use dicom_dictionary_std::uids;
use crate::{DecodeError, PhotometricInterpretation, PixelDecoder, PixelRepresentation, PixelValues, PlanarConfiguration};
use crate::DecodeError::UnsupportedNumberOfBits;

#[derive(Clone, Debug, Default)]
pub struct RawPixelDecoder {

}

impl<'a> PixelDecoder<DecodingData<'a>> for RawPixelDecoder {
    fn decode(&self, params: &DecodingData<'a>) -> Result<PixelValues, DecodeError> {
        match params.transfer_syntax.as_str() {
            uids::EXPLICIT_VR_LITTLE_ENDIAN | uids::IMPLICIT_VR_LITTLE_ENDIAN => { decode_little_endian(params) }
            &_ => {
                Err(DecodeError::UnsupportedTransferSyntax(params.transfer_syntax.clone()))
            }
        }
    }
}

/// Struct representing the decoding data of an image.
///
/// The decoding data includes information about the image data, such as its structure and interpretation,
/// as well as various parameters used for decoding and rescaling the pixel values.
///
/// # Fields
///
/// - `data`: A borrowed slice of bytes representing the image pixel data.
/// - `rows`: The number of rows in the image.
/// - `columns`: The number of columns in the image.
/// - `number_of_frames`: The number of frames in the image.
/// - `photometric_interpretation`: The photometric interpretation of the image, indicating how the pixel
///   values should be interpreted.
/// - `samples_per_pixel`: The number of samples per pixel in the image.
/// - `planar_configuration`: The planar configuration of the image, indicating how the samples are
///   organized.
/// - `bits_allocated`: The number of bits allocated for each pixel.
/// - `bits_stored`: The number of bits actually used for each pixel.
/// - `high_bit`: The highest bit position used for the pixel values.
/// - `pixel_representation`: The pixel representation, indicating whether the pixel values are signed or unsigned.
/// - `rescale_intercept`: The intercept value used for rescaling the pixel values.
/// - `rescale_slope`: The slope value used for rescaling the pixel values.
pub struct DecodingData<'a> {
    transfer_syntax: String,
    data: Cow<'a, [u8]>,
    rows: u32,
    columns: u32,
    number_of_frames: u32,
    photometric_interpretation: PhotometricInterpretation,
    samples_per_pixel: u16,
    planar_configuration: PlanarConfiguration,
    bits_allocated: u16,
    bits_stored: u16,
    high_bit: u16,
    pixel_represenation: PixelRepresentation,
    rescale_intercept: f64,
    rescale_slope: f64,
}

/// Decode the given data in little endian format.
///
/// # Arguments
///
/// * `data` - The decoding data containing the pixel data and decoding parameters.
///
/// # Returns
///
/// A `Result` that contains the decoded pixel values on success or an error on failure.
///
/// # Errors
///
/// * `DecodeError::UnsupportedPhotometricInterpretation` - If the photometric interpretation is not supported.
fn decode_little_endian(data: &DecodingData) -> Result<PixelValues, DecodeError> {
    match data.photometric_interpretation {
        PhotometricInterpretation::Monochrome1 | PhotometricInterpretation::Monochrome2 => { decode_little_endian_monochrome(data) }
        _ => { Err(DecodeError::UnsupportedPhotometricInterpretation(data.photometric_interpretation.as_ref().to_string())) }
    }
}

/// Decode little endian monochrome pixel data.
///
/// This function decodes monochrome (unsigned and signed) pixel data in little endian byte order. It supports
/// decoding data with different pixel representations and number of bits allocated per pixel.
///
/// # Arguments
///
/// * `data` - The decoding data containing the pixel data and decoding parameters.
///
/// # Errors
///
/// This function can return the following errors:
///
/// * `DecodeError::ExpectedByteMismatch(n, m)` - When the number of bytes in the input data does
///   not match the expected number of bytes calculated based on the decoding parameters.
/// * `DecodeError::UnsupportedNumberOfBits(bits)` - When the number of bits allocated per pixel
///   is not supported by this function.
fn decode_little_endian_monochrome(data: &DecodingData) -> Result<PixelValues, DecodeError> {
    let num_pixels = (data.rows * data.columns * data.number_of_frames) as usize;
    let num_samples = num_pixels * data.samples_per_pixel as usize;

    let nbytes = num_samples * data.bits_allocated as usize / 8;
    if nbytes != data.data.len() {
        return Err(DecodeError::ExpectedByteMismatch(nbytes, data.data.len()));
    }

    match data.pixel_represenation {
        PixelRepresentation::UnsignedInteger => {
            match data.bits_allocated {
                8 => {
                    let n = data.data.len();
                    let mut tv = Vec::with_capacity(num_samples);
                    let mask = u8_mask(data.high_bit as u8)?;
                    for i in 0..n {
                        let b = data.data[i];
                        tv.push(b & mask);
                    }
                    Ok(PixelValues::U8(tv))
                }
                16 => {
                    let n = data.data.len();
                    let mut i = 0;
                    let mut tv = Vec::with_capacity(num_samples);
                    let mask = u16_mask(data.high_bit)?;
                    while i < n {
                        let b0 = data.data[i];
                        let b1 = data.data[i + 1];
                        let value = u16::from_le_bytes([b0, b1]);
                        tv.push(value & mask);
                        i += 2;
                    }
                    Ok(PixelValues::U16(tv))
                }
                _ => {
                    Err(UnsupportedNumberOfBits(data.bits_allocated as usize))
                }
            }
        }
        PixelRepresentation::TwosComplement => {
            match data.bits_allocated {
                8 => {
                    let mut tv = vec![];
                    let n = data.data.len();
                    let mask = u8_mask(data.high_bit as u8)?;
                    for i in 0..n {
                        let b = data.data[i];
                        let value = i8::from_le_bytes([b]);
                        tv.push(((value as u8) & mask) as i8);
                    }
                    Ok(PixelValues::I8(tv))
                }
                16 => {
                    let n = data.data.len();
                    let mut i = 0;
                    let mut tv = vec![];
                    let mask = u16_mask(data.high_bit)?;
                    while i < n {
                        let b0 = data.data[i];
                        let b1 = data.data[i + 1];
                        let value = i16::from_le_bytes([b0, b1]);
                        tv.push(((value as u16) & mask) as i16);
                        i += 2;
                    }
                    Ok(PixelValues::I16(tv))
                }
                _ => {
                    Err(UnsupportedNumberOfBits(data.bits_allocated as usize))
                }
            }
        }
    }
}

/// Creates a bitmask with the high bit set to the specified position.
///
/// # Arguments
///
/// * `high_bit` - The position of the high bit in the bitmask. Must be less than 8.
///
/// # Returns
///
/// Returns a `Result` with the bitmask if the operation was successful, or an error if `high_bit` is out of bounds.
///
/// # Errors
///
/// Returns a `DecodeError` if the `high_bit` is greater than or equal to 8.
fn u8_mask(high_bit: u8) -> Result<u8, DecodeError> {
    if high_bit >= 8 {
        return Err(DecodeError::U8MaskHighBitOutOfBound(high_bit));
    }
    let mut mask = 0u8;
    for i in 0..high_bit {
        mask |= 1 << i;
    }
    Ok(mask)
}

/// Creates a bitmask with the high bit set to the specified position.
///
/// # Arguments
///
/// * `high_bit` - The position of the high bit in the bitmask. Must be less than 16.
///
/// # Returns
///
/// Returns a `Result` with the bitmask if the operation was successful, or an error if `high_bit` is out of bounds.
///
/// # Errors
///
/// Returns a `DecodeError` if the `high_bit` is greater than or equal to 16.
fn u16_mask(high_bit: u16) -> Result<u16, DecodeError> {
    if high_bit >= 16 {
        return Err(DecodeError::U16MaskHighBitOutOfBound(high_bit));
    }
    let mut mask = 0u16;
    for i in 0..high_bit {
        mask |= 1 << i;
    }
    Ok(mask)
}

#[cfg(test)]
mod tests {
    use dicom_dictionary_std::uids::IMPLICIT_VR_LITTLE_ENDIAN;

    use super::*;

    #[test]
    fn u8_mask_0() {
        assert_eq!(u8_mask(0).unwrap(), 0u8);
    }

    #[test]
    fn u8_mask_1() {
        assert_eq!(u8_mask(1).unwrap(), 1u8);
    }

    #[test]
    fn u8_mask_2() {
        assert_eq!(u8_mask(2).unwrap(), 3u8);
    }

    #[test]
    fn u8_mask_3() {
        assert_eq!(u8_mask(3).unwrap(), 7u8);
    }

    #[test]
    fn u8_mask_out_of_bounds() {
        match u8_mask(8) {
            Ok(_) => panic!("Expected error, but got Ok(_)!"),
            Err(e) => assert_eq!(e, DecodeError::U8MaskHighBitOutOfBound(8)),
        }
    }

    #[test]
    fn u16_mask_0() {
        assert_eq!(u16_mask(0).unwrap(), 0u16);
    }

    #[test]
    fn u16_mask_1() {
        assert_eq!(u16_mask(1).unwrap(), 1u16);
    }

    #[test]
    fn u16_mask_2() {
        assert_eq!(u16_mask(2).unwrap(), 3u16);
    }

    #[test]
    fn u16_mask_3() {
        assert_eq!(u16_mask(3).unwrap(), 7u16);
    }

    #[test]
    fn u16_mask_out_of_bounds() {
        match u16_mask(16) {
            Ok(_) => panic!("Expected error, but got Ok(_)!"),
            Err(e) => assert_eq!(e, DecodeError::U16MaskHighBitOutOfBound(16)),
        }
    }

    #[test]
    fn monochrome_u16bit() {
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
            rescale_slope: 1.0,
        };

        match decode_little_endian(&mock_data) {
            Ok(pixel_values) => {
                match pixel_values {
                    PixelValues::U16(pixels) => {
                        let expected = [
                            0u16,
                            0x1c1e,
                            0x1c1f,
                            0x1c1a,
                            0x1c1c,
                            0x1c1c,
                            0x1c1d,
                            0x1c1b
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