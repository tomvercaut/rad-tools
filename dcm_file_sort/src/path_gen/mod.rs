mod default_gen;
mod factory;
mod types;
mod unkown;
mod uzg;

pub use factory::DicomDirPathGeneratorFactory;
pub use types::*;
pub use unkown::*;

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum SortedPathGeneratorError {
    #[error("Unable to create a unique directory path because the patient ID is empty.")]
    DicomPathGeneratorPatientIdEmpty,
    #[error(
        "Supported DICOM path generators are: DefaultDicomPathGenerator, UZGDicomPathGenerator. The provided path generator is not supported for DICOM data."
    )]
    UnsupportedDicomPathGenerator,
}

/// A trait for generating directory paths for sorting data.
pub trait SortedDirPathGenerator {
    type SortedPathGeneratorError;
    /// Returns a PathBuf representing the path for the sorted data.
    ///
    /// # Returns
    /// * `PathBuf` - The generated path where data should be sorted into. This path can be full or relative.
    fn sort_dir_path(&self) -> Result<PathBuf, Self::SortedPathGeneratorError>;
}
