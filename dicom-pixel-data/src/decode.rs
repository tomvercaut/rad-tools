use std::borrow::Cow;

use crate::{PhotometricInterpretation, PixelRepresentation, PlanarConfiguration};

#[derive(thiserror::Error, Debug)]
enum DecodeError {}

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
    data: Cow<'a, [u8]>,
    rows: u32,
    columns: u32,
    number_of_frames: u32,
    photometric_interpretation: PhotometricInterpretation,
    samples_per_pixel: u16,
    planar_configuratino: PlanarConfiguration,
    bits_allocated: u16,
    bits_stored: u16,
    high_bit: u16,
    pixel_represenation: PixelRepresentation,
    rescale_intercept: f64,
    rescale_slope: f64,
}

pub fn decode(data: &DecodingData) -> Result<Vec<i64>, DecodeError> {
    unimplemented!()
}