use crate::path_gen::default_gen::DefaultDicomArchiveDirPathGenerator;
use crate::path_gen::{SortedDirPathGenerator, SortedPathGeneratorError};
use crate::{DicomData, parse_date};
use dicom_core::chrono::Datelike;
use std::path::PathBuf;

pub struct UZGDicomArchiveDirPathGenerator<'a> {
    dicom_data: &'a DicomData,
}

impl<'a> UZGDicomArchiveDirPathGenerator<'a> {
    pub fn new(dicom_data: &'a DicomData) -> Self {
        Self { dicom_data }
    }
}

impl SortedDirPathGenerator for UZGDicomArchiveDirPathGenerator<'_> {
    type SortedPathGeneratorError = SortedPathGeneratorError;

    /// Generates an archive path based on the date of birth or the patient ID.
    ///
    /// This implementation creates a path structure of `MMDD/PatientID` where:
    /// * `MMDD` is derived from the patient's date of birth
    /// * `PatientID` is the patient's unique identifier
    ///
    /// If the date of birth is not directly available in the DICOM metadata,
    /// it attempts to extract the date of birth from the first 6 characters
    /// of the patient ID by prepending "00" and parsing it as a date.
    ///
    /// If no valid date of birth can be determined, it falls back to using the
    /// DefaultDicomArchivePathGenerator implementation.
    ///
    /// # Returns
    /// * `Ok(PathBuf)` - A path in the format `MMDD/PatientID`
    /// * `Err(Error)` - If the patient ID is empty or date parsing fails
    fn sort_dir_path(&self) -> Result<PathBuf, Self::SortedPathGeneratorError> {
        let pid = self.dicom_data.patient_id.trim();
        let dob = match &self.dicom_data.date_of_birth {
            None => {
                if pid.len() >= 6 {
                    let t = format!("00{}", &pid[0..6]);
                    parse_date(&t).ok()
                } else {
                    None
                }
            }
            Some(dob) => Some(*dob),
        };
        // Fallback to the default path generator
        let is_none = dob.is_none();
        if is_none {
            return DefaultDicomArchiveDirPathGenerator::new(self.dicom_data).sort_dir_path();
        }
        let dob = dob.unwrap();

        let month = if dob.month() < 10 {
            format!("0{}", dob.month())
        } else {
            format!("{}", dob.month())
        };
        let day = if dob.day() < 10 {
            format!("0{}", dob.day())
        } else {
            format!("{}", dob.day())
        };
        let month_day = format!("{}{}", month, day);
        let output_path = PathBuf::from(month_day).join(&self.dicom_data.patient_id);
        Ok(output_path)
    }
}

#[cfg(test)]
mod tests {
    use crate::DicomData;
    use crate::path_gen::SortedDirPathGenerator;
    use crate::path_gen::uzg::UZGDicomArchiveDirPathGenerator;
    use chrono::NaiveDate;
    use std::path::PathBuf;

    #[test]
    fn test_uzg_generator_with_date_of_birth() {
        let dicom_data = DicomData {
            patient_id: "12345".to_string(),
            date_of_birth: Some(NaiveDate::from_ymd_opt(1985, 6, 15).unwrap()),
            ..Default::default()
        };
        let generator = UZGDicomArchiveDirPathGenerator::new(&dicom_data);
        let result = generator.sort_dir_path().unwrap();
        assert_eq!(result, PathBuf::from("0615/12345"));
    }

    #[test]
    fn test_uzg_generator_with_date_in_patient_id() {
        let dicom_data = DicomData {
            patient_id: "850615ABC".to_string(),
            date_of_birth: None,
            ..Default::default()
        };
        let generator = UZGDicomArchiveDirPathGenerator::new(&dicom_data);
        let result = generator.sort_dir_path().unwrap();
        assert_eq!(result, PathBuf::from("0615/850615ABC"));
    }

    #[test]
    fn test_uzg_generator_fallback_to_default() {
        let dicom_data = DicomData {
            patient_id: "12345".to_string(),
            date_of_birth: None,
            ..Default::default()
        };
        let generator = UZGDicomArchiveDirPathGenerator::new(&dicom_data);
        let result = generator.sort_dir_path().unwrap();
        assert_eq!(result, PathBuf::from("patient_id/12345"));
    }
}
