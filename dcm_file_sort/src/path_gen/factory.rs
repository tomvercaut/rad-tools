use crate::DicomData;
use crate::path_gen::default_gen::DefaultDicomArchiveDirPathGenerator;
use crate::path_gen::uzg::UZGDicomArchiveDirPathGenerator;
use crate::path_gen::{DicomPathGeneratorType, SortedDirPathGenerator, SortedPathGeneratorError};
use std::path::PathBuf;

pub struct DicomDirPathGeneratorFactory<'a> {
    gtype: DicomPathGeneratorType,
    data: &'a DicomData,
}

impl<'a> DicomDirPathGeneratorFactory<'a> {
    pub fn new(gtype: DicomPathGeneratorType, data: &'a DicomData) -> Self {
        Self { gtype, data }
    }
}

impl<'a> SortedDirPathGenerator for DicomDirPathGeneratorFactory<'a> {
    type SortedPathGeneratorError = SortedPathGeneratorError;

    fn sort_dir_path(&self) -> Result<PathBuf, Self::SortedPathGeneratorError> {
        match self.gtype {
            DicomPathGeneratorType::Default => {
                DefaultDicomArchiveDirPathGenerator::new(self.data).sort_dir_path()
            }
            DicomPathGeneratorType::Uzg => {
                UZGDicomArchiveDirPathGenerator::new(self.data).sort_dir_path()
            }
        }
    }
}
