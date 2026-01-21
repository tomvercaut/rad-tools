use crate::Error;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::str::FromStr;
use tracing::error;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum DicomPathGeneratorType {
    #[default]
    #[serde(rename = "dicom_default")]
    Default,
    #[serde(rename = "dicom_uzg")]
    Uzg,
}

impl FromStr for DicomPathGeneratorType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dicom_default" => Ok(DicomPathGeneratorType::Default),
            "dicom_uzg" => Ok(DicomPathGeneratorType::Uzg),
            _ => {
                error!("Invalid path generator type: {}", s);
                Err(Error::InvalidFromStrToPathGeneratorType)
            }
        }
    }
}

impl Debug for DicomPathGeneratorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Display for DicomPathGeneratorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DicomPathGeneratorType::Default => "dicom_default".to_string(),
            DicomPathGeneratorType::Uzg => "dicom_uzg".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_path_generator_type() {
        assert_eq!(
            serde_json::to_string(&DicomPathGeneratorType::Default).unwrap(),
            "\"dicom_default\""
        );
        assert_eq!(
            serde_json::to_string(&DicomPathGeneratorType::Uzg).unwrap(),
            "\"dicom_uzg\""
        );
    }

    #[test]
    fn test_deserialize_path_generator_type() {
        assert_eq!(
            serde_json::from_str::<DicomPathGeneratorType>("\"dicom_default\"").unwrap(),
            DicomPathGeneratorType::Default
        );
        assert_eq!(
            serde_json::from_str::<DicomPathGeneratorType>("\"dicom_uzg\"").unwrap(),
            DicomPathGeneratorType::Uzg
        );
    }

    #[test]
    fn test_deserialize_invalid_path_generator_type() {
        assert!(serde_json::from_str::<DicomPathGeneratorType>("\"invalid\"").is_err());
    }
}
