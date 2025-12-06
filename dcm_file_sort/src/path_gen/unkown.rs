use crate::path_gen::{SortedDirPathGenerator, SortedPathGeneratorError};
use std::path::{Path, PathBuf};

pub struct UnknownDataArchiveDirPathGenerator<'a> {
    dir: &'a Path,
}

impl<'a> UnknownDataArchiveDirPathGenerator<'a> {
    pub fn new(dir: &'a Path) -> Self {
        Self { dir }
    }
}

impl<'a> SortedDirPathGenerator for UnknownDataArchiveDirPathGenerator<'a> {
    type SortedPathGeneratorError = SortedPathGeneratorError;

    fn sort_dir_path(&self) -> Result<PathBuf, Self::SortedPathGeneratorError> {
        Ok(self.dir.into())
    }
}
