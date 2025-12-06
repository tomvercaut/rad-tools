use crate::DicomData;
use crate::path_gen::{SortedDirPathGenerator, SortedPathGeneratorError};
use std::path::PathBuf;

pub struct DefaultDicomArchiveDirPathGenerator<'a> {
    dicom_data: &'a DicomData,
}

impl<'a> DefaultDicomArchiveDirPathGenerator<'a> {
    pub fn new(dicom_data: &'a DicomData) -> Self {
        Self { dicom_data }
    }
}

impl SortedDirPathGenerator for DefaultDicomArchiveDirPathGenerator<'_> {
    type SortedPathGeneratorError = SortedPathGeneratorError;
    fn sort_dir_path(&self) -> Result<PathBuf, Self::SortedPathGeneratorError> {
        let pid = self.dicom_data.patient_id.trim();
        if pid.is_empty() {
            return Err(SortedPathGeneratorError::DicomPathGeneratorPatientIdEmpty);
        }
        Ok(PathBuf::from(format!("patient_id/{}", pid)))
    }
}

#[cfg(test)]
mod tests {
    use crate::DicomData;
    use crate::path_gen::default_gen::DefaultDicomArchiveDirPathGenerator;
    use crate::path_gen::{SortedDirPathGenerator, SortedPathGeneratorError};
    use std::path::PathBuf;

    #[test]
    fn test_default_generator_valid_patient_id() {
        let dicom_data = DicomData {
            patient_id: "12345".to_string(),
            ..Default::default()
        };
        let generator = DefaultDicomArchiveDirPathGenerator::new(&dicom_data);
        let result = generator.sort_dir_path().unwrap();
        assert_eq!(result, PathBuf::from("patient_id/12345"));
    }

    #[test]
    fn test_default_generator_empty_patient_id() {
        let dicom_data = DicomData::default();
        let generator = DefaultDicomArchiveDirPathGenerator::new(&dicom_data);
        let result = generator.sort_dir_path();
        assert!(matches!(
            result,
            Err(SortedPathGeneratorError::DicomPathGeneratorPatientIdEmpty)
        ));
    }
}
