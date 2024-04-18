use derive_builder::{Builder, UninitializedFieldError};
use num_traits::Num;

#[derive(Clone, Debug, Builder)]
#[builder(build_fn(validate = "check_image", error = "ImageBuildValidationError"))]
pub struct Image<T> where T: Num + Clone {
    /// Represents the dimensions of an image.
    ///
    /// The `dims` field is a `Vec<usize>` which stores the dimensions
    /// of the image along each axis. The length of the `Vec` indicates
    /// the number of axes, and each element represents the size of the
    /// corresponding axis.
    ///
    /// For example, if `dims` is `[width, height]`, then the image has
    /// dimensions `width` pixels wide and `height` pixels high.
    dims: Vec<usize>,
    /// The origin of the image or coordinates in ND space.
    /// Represents the coordinates (x, y, ...) of the top-left corner of the image.
    /// The origin is specified in floating-point numbers.
    origin: Vec<f64>,
    /// Image axis spacing in physical units.
    spacing: Vec<f64>,
    /// # pixels
    ///
    /// The `pixels` field represents the list of individual pixels in an image.
    ///
    /// The type parameter `T` represents the type of each pixel.
    pixels: Vec<T>,
}

impl<T> Image<T> where T: Num + Clone {
    pub fn dims(&self) -> &[usize] {
        &self.dims
    }

    pub fn origin(&self) -> &[f64] {
        &self.origin
    }
    pub fn spacing(&self) -> &[f64] {
        &self.spacing
    }
    pub fn pixels(&self) -> &[T] {
        &self.pixels
    }
    pub fn len(&self) -> usize {
        self.pixels.len()
    }
    pub fn is_empty(&self) -> bool {
        self.pixels.is_empty()
    }
}

impl<T> IntoIterator for Image<T> where T: Num + Clone {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels.into_iter()
    }
}

#[derive(thiserror::Error, Clone, Debug)]
pub enum ImageBuildValidationError {
    #[error("UninitializedField: {0}")]
    UninitializedField(&'static str),
    #[error("No dimensions were defined")]
    EmptyDimensions,
    #[error("No origin was defined")]
    EmptyOrigin,
    #[error("No spacings were defined")]
    EmptySpacing,
    #[error("Number of pixels computed from the set dimensions [{0:#?}] doens't match with the number of set pixels [{1}].")]
    DimensionPixelsLen(Vec<usize>, usize),
    #[error("Number of dimensions of the image [{0}] doesn't match with the number of dimensions on the origin [{1}].")]
    ImageDimensionsVsOrigin(usize, usize),
    #[error("Number of dimensions of the image [{0}] doesn't match with the number of dimensions on the spacing [{1}].")]
    ImageDimensionsVsSpacing(usize, usize),
}

impl From<UninitializedFieldError> for ImageBuildValidationError {
    fn from(error: UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

/// Checks the validity of an image build using the given `builder`.
///
/// # Arguments
///
/// * `builder` - The image builder to be checked.
///
/// # Returns
///
/// * `Ok(())` if the image build is valid.
/// * `Err` with a specific `ImageBuildValidationError` if the image build is not valid.
///
/// # Errors
///
/// The function may return the following errors:
///
/// * `EmptyDimensions` - The dimensions of the image to construct are empty.
/// * `ImageDimensionsVsOrigin` - The number of dimensions in the image builder does not match the number of origin coordinates.
/// * `ImageDimensionsVsSpacing` - The number of dimensions in the image builder does not match the number of spacing values.
/// * `EmptyDimensions` - The builder has image dimensions with a total number of pixels equal to zero.
/// * `DimensionPixelsLen` - The number of pixels in the builder does not match the expected number calculated from the dimensions.
fn check_image<T: Num + Clone>(builder: &ImageBuilder<T>) -> Result<(), ImageBuildValidationError> {
    if let Some(dims) = &builder.dims {
        let ndims = dims.len();
        if ndims == 0 {
            return Err(ImageBuildValidationError::EmptyDimensions);
        }

        if let Some(origin) = &builder.origin {
            if origin.is_empty() {
                return Err(ImageBuildValidationError::EmptyOrigin);
            }
            if origin.len() != ndims {
                return Err(ImageBuildValidationError::ImageDimensionsVsOrigin(dims.len(), origin.len()));
            }
        }
        if let Some(spacing) = &builder.spacing {
            if spacing.is_empty() {
                return Err(ImageBuildValidationError::EmptySpacing);
            }
            if spacing.len() != ndims {
                return Err(ImageBuildValidationError::ImageDimensionsVsSpacing(dims.len(), spacing.len()));
            }
        }

        let mut npixels = 1;
        for dim in dims {
            npixels *= dim;
        }
        if npixels == 0 {
            return Err(ImageBuildValidationError::EmptyDimensions);
        }
        if let Some(pixels) = &builder.pixels {
            if pixels.len() != npixels {
                return Err(ImageBuildValidationError::DimensionPixelsLen(dims.clone(), pixels.len()));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::dicom::model::{Image, ImageBuilder, ImageBuildValidationError};

    #[test]
    fn image_build() {
        let r = ImageBuilder::default()
            .dims(vec![2, 3])
            .origin(vec![200.0, 300.0])
            .spacing(vec![0.2, 0.4])
            .pixels(vec![1, 2, 3, 4, 5, 6]).build();
        assert!(r.is_ok());
        let image: Image<i32> = r.unwrap();
        assert_eq!(&[2, 3], image.dims());
        assert_eq!(&[200.0, 300.0], image.origin());
        assert_eq!(&[0.2, 0.4], image.spacing());
        assert_eq!(&[1, 2, 3, 4, 5, 6], image.pixels());
    }

    #[test]
    fn image_build_uninitialized_field() {
        let r = ImageBuilder::default()
            .origin(vec![200.0, 300.0])
            .spacing(vec![0.2, 0.4])
            .pixels(vec![1, 2, 3, 4, 5, 6]).build();
        assert!(r.is_err());
        match r.unwrap_err() {
            ImageBuildValidationError::UninitializedField(field) => assert_eq!("dims", field),
            _ => assert!(false),
        }
    }

    #[test]
    fn image_build_empty_dim() {
        let r = ImageBuilder::default()
            .dims(vec![])
            .origin(vec![200.0, 300.0])
            .spacing(vec![0.2, 0.4])
            .pixels(vec![1, 2, 3, 4, 5, 6]).build();
        assert!(r.is_err());
        match r.unwrap_err() {
            ImageBuildValidationError::EmptyDimensions => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn image_build_empty_origin() {
        let r = ImageBuilder::default()
            .dims(vec![2, 3])
            .origin(vec![])
            .spacing(vec![0.2, 0.4])
            .pixels(vec![1, 2, 3, 4, 5, 6]).build();
        assert!(r.is_err());
        match r.unwrap_err() {
            ImageBuildValidationError::EmptyOrigin => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn image_build_mismatch_origin_dimension() {
        let r = ImageBuilder::default()
            .dims(vec![2, 3])
            .origin(vec![200.0])
            .spacing(vec![0.2, 0.4])
            .pixels(vec![1, 2, 3, 4, 5, 6]).build();
        assert!(r.is_err());
        match r.unwrap_err() {
            ImageBuildValidationError::ImageDimensionsVsOrigin(2, 1) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn image_build_empty_spacing() {
        let r = ImageBuilder::default()
            .dims(vec![2, 3])
            .origin(vec![200.0, 300.0])
            .spacing(vec![])
            .pixels(vec![1, 2, 3, 4, 5, 6]).build();
        assert!(r.is_err());
        match r.unwrap_err() {
            ImageBuildValidationError::EmptySpacing => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn image_build_mismatch_spacing_dimension() {
        let r = ImageBuilder::default()
            .dims(vec![2, 3])
            .origin(vec![200.0, 300.0])
            .spacing(vec![0.2])
            .pixels(vec![1, 2, 3, 4, 5, 6]).build();
        assert!(r.is_err());
        match r.unwrap_err() {
            ImageBuildValidationError::ImageDimensionsVsSpacing(2, 1) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn image_build_mismatch_pixel_len() {
        let r = ImageBuilder::default()
            .dims(vec![2, 3])
            .origin(vec![200.0, 300.0])
            .spacing(vec![0.2, 0.4])
            .pixels(vec![1, 2, 3, 4, 5]).build();
        assert!(r.is_err());
        match r.unwrap_err() {
            ImageBuildValidationError::DimensionPixelsLen(_dims, _u) => assert!(true),
            _ => assert!(false),
        }
    }
}