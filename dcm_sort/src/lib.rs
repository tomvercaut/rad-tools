use std::path::{Path, PathBuf};

const STUDY_INSTANCE_UID_UNKNOWN: &str = "STUDY_UID_UNKNOWN";
const SERIES_INSTANCE_UID_UNKNOWN: &str = "SERIES_UID_UNKNOWN";
const SERIES_NUMBER_UNKNOWN: &str = "SERIES_NUMBER_UNKNOWN";
const MODALITY_UNKNOWN: &str = "MODALITY_UNKNOWN";

#[derive(Debug)]
pub enum Error {
    PatientIdUnknown,
}

pub type Result<T> = std::result::Result<T, Error>;

/// Data by which the DICOM files can be categorized.
#[derive(Clone, Debug)]
pub struct Data {
    /// Unique patient identifier
    pub patient_id: String,
    /// Study Instance UID
    pub study_uid: String,
    /// Study description
    pub study_descr: String,
    /// Series Instance UID
    pub series_uid: String,
    /// Series description
    pub series_descr: String,
    /// Series number
    pub series_nr: String,
    /// Modality
    pub modality: String,
}

impl Data {
    /// Create a file path based on the data.
    ///
    /// Format of the path being created:
    /// <p>/<study>/<series>/<series nr>/<modality>
    ///
    /// where:
    /// - `p`: input path
    /// - `study`:
    ///     - study description, if not empty
    ///     - study instance UID, if not empty
    ///     - STUDY_UID_UNKNOWN
    /// - `series`:
    ///     - series description, if not empty
    ///     - series instance UID, if not empty
    ///     - SERIES_UID_UNKNOWN
    /// - `series nr`:
    ///     - series number, if not empty
    ///     - SERIES_UID_UNKNOWN
    /// - `modality`:
    ///     - modality, if not empty
    ///     - MODALITY_UNKNOWN
    pub fn to_path_buf<P>(&self, p: P) -> Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        if self.patient_id.trim().is_empty() {
            return Err(Error::PatientIdUnknown);
        }
        let p = p.as_ref();
        let pb = p
            .join(&self.patient_id)
            .join(
                if self.study_uid.is_empty() && self.study_descr.is_empty() {
                    STUDY_INSTANCE_UID_UNKNOWN
                } else if !self.study_descr.is_empty() {
                    &self.study_descr
                } else {
                    &self.study_uid
                },
            )
            .join(
                if self.series_uid.is_empty() && self.series_descr.is_empty() {
                    SERIES_INSTANCE_UID_UNKNOWN
                } else if !self.series_descr.is_empty() {
                    &self.series_descr
                } else {
                    &self.series_uid
                },
            )
            .join(if self.series_nr.is_empty() {
                SERIES_NUMBER_UNKNOWN
            } else {
                &self.series_nr
            })
            .join(if self.modality.is_empty() {
                MODALITY_UNKNOWN
            } else {
                &self.modality
            });
        Ok(pb)
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{
        Data, MODALITY_UNKNOWN, SERIES_INSTANCE_UID_UNKNOWN, SERIES_NUMBER_UNKNOWN,
        STUDY_INSTANCE_UID_UNKNOWN,
    };

    #[test]
    fn to_path_buf() {
        let datas = [
            Data {
                patient_id: "pt_id".into(),
                study_uid: "study".into(),
                study_descr: "study_descr".into(),
                series_uid: "series".into(),
                series_descr: "series_descr".into(),
                series_nr: "1".into(),
                modality: "CT".into(),
            },
            Data {
                patient_id: "pt_id".into(),
                study_uid: "study".into(),
                study_descr: "".into(),
                series_uid: "series".into(),
                series_descr: "".into(),
                series_nr: "1".into(),
                modality: "CT".into(),
            },
            Data {
                patient_id: "pt_id".into(),
                study_uid: "".into(),
                study_descr: "".into(),
                series_uid: "".into(),
                series_descr: "".into(),
                series_nr: "".into(),
                modality: "".into(),
            },
        ];
        let bufs = [
            PathBuf::new()
                .join(".")
                .join("pt_id")
                .join("study_descr")
                .join("series_descr")
                .join("1")
                .join("CT"),
            PathBuf::new()
                .join(".")
                .join("pt_id")
                .join("study")
                .join("series")
                .join("1")
                .join("CT"),
            PathBuf::new()
                .join(".")
                .join("pt_id")
                .join(STUDY_INSTANCE_UID_UNKNOWN)
                .join(SERIES_INSTANCE_UID_UNKNOWN)
                .join(SERIES_NUMBER_UNKNOWN)
                .join(MODALITY_UNKNOWN),
        ];
        assert_eq!(bufs.len(), datas.len())
    }
}
