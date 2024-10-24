use crate::{PersonName, PhotometricInterpretation, PixelRepresentation, Sop};
use chrono::{NaiveDate, NaiveDateTime};
use std::str::FromStr;

#[derive(Clone, Debug, Default)]
pub struct RTDose {
    pub specific_character_set: String,
    pub image_type: Vec<String>,
    pub sop: Sop,
    pub study_dt: NaiveDateTime,
    pub content_dt: NaiveDateTime,
    pub accession_number: String,
    pub modality: String,
    pub ref_physician_name: PersonName,
    pub station_name: String,
    pub manufacturer_model_name: String,
    pub irradiation_event_uid: String,
    pub patient_name: PersonName,
    pub patient_id: String,
    pub patient_birth_date: NaiveDate,
    pub patient_sex: String,
    pub patient_identity_removed: String,
    pub software_versions: String,
    pub study_instance_uid: String,
    pub series_instance_uid: String,
    pub study_id: String,
    pub series_number: i32,
    pub instance_number: i32,
    pub image_position_patient: [f64; 3],
    pub image_orientation_patient: [f64; 6],
    pub frame_of_reference_uid: String,
    pub position_reference_indicator: String,
    pub samples_per_pixel: i32,
    pub photometric_interpretation: PhotometricInterpretation,
    pub number_of_frames: i32,
    pub frame_increment_pointer: String,
    pub rows: i32,
    pub columns: i32,
    pub pixel_spacing: [f64; 2],
    pub bits_allocated: i32,
    pub bits_stored: i32,
    pub high_bit: i32,
    pub pixel_representation: PixelRepresentation,
    pub dose_units: DoseUnits,
    pub dose_type: DoseType,
    pub dose_comment: String,
    pub dose_summation_type: DoseSummationType,
    pub grid_frame_offset_vector: Vec<f64>,
    pub dose_grid_scaling: f64,
    pub tissue_heterogeneity_correction: TissueHeterogeneityCorrection,
    pub referenced_rt_plan_sequence: Vec<Sop>,
    pub pixel_data: Vec<u8>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum DoseUnits {
    #[default]
    NONE,
    GY,
    RELATIVE,
}

#[derive(Debug, thiserror::Error)]
pub enum DoseUnitsError {
    #[error("'{0}' is not a valid DoseUnits value")]
    InvalidValue(String),
}

impl FromStr for DoseUnits {
    type Err = DoseUnitsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GY" => Ok(DoseUnits::GY),
            "RELATIVE" => Ok(DoseUnits::RELATIVE),
            _ => Err(DoseUnitsError::InvalidValue(s.to_string())),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum DoseType {
    #[default]
    NONE,
    PHYSICAL,
    EFFECTIVE,
    ERROR,
}

#[derive(Debug, thiserror::Error)]
pub enum DoseTypeError {
    #[error("'{0}' is not a valid DoseType value")]
    InvalidValue(String),
}

impl FromStr for DoseType {
    type Err = DoseTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PHYSICAL" => Ok(DoseType::PHYSICAL),
            "EFFECTIVE" => Ok(DoseType::EFFECTIVE),
            "ERROR" => Ok(DoseType::ERROR),
            _ => Err(DoseTypeError::InvalidValue(s.to_string())),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DoseSummationType {
    #[default]
    NONE,
    PLAN,
    MULTI_PLAN,
    PLAN_OVERVIEW,
    FRACTION,
    BEAM,
    BRACHY,
    FRACTION_SESSION,
    BEAM_SESSION,
    BRACHY_SESSION,
    CONTROL_POINT,
    RECORD,
}

#[derive(Debug, thiserror::Error)]
pub enum DoseSummationTypeError {
    #[error("'{0}' is not a valid DoseSummationType value")]
    InvalidValue(String),
}

impl FromStr for DoseSummationType {
    type Err = DoseSummationTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PLAN" => Ok(DoseSummationType::PLAN),
            "MULTI_PLAN" => Ok(DoseSummationType::MULTI_PLAN),
            "PLAN_OVERVIEW" => Ok(DoseSummationType::PLAN_OVERVIEW),
            "FRACTION" => Ok(DoseSummationType::FRACTION),
            "BEAM" => Ok(DoseSummationType::BEAM),
            "BRACHY" => Ok(DoseSummationType::BRACHY),
            "FRACTION_SESSION" => Ok(DoseSummationType::FRACTION_SESSION),
            "BEAM_SESSION" => Ok(DoseSummationType::BEAM_SESSION),
            "BRACHY_SESSION" => Ok(DoseSummationType::BRACHY_SESSION),
            "CONTROL_POINT" => Ok(DoseSummationType::CONTROL_POINT),
            "RECORD" => Ok(DoseSummationType::RECORD),
            _ => Err(DoseSummationTypeError::InvalidValue(s.to_string())),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TissueHeterogeneityCorrection {
    #[default]
    NONE,
    IMAGE,
    ROI_OVERRIDE,
    WATER,
}

#[derive(Debug, thiserror::Error)]
pub enum TissueHeterogeneityCorrectionError {
    #[error("'{0}' is not a valid TissueHeterogeneityCorrection value")]
    InvalidValue(String),
}

impl FromStr for TissueHeterogeneityCorrection {
    type Err = TissueHeterogeneityCorrectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "IMAGE" => Ok(TissueHeterogeneityCorrection::IMAGE),
            "ROI_OVERRIDE" => Ok(TissueHeterogeneityCorrection::ROI_OVERRIDE),
            "WATER" => Ok(TissueHeterogeneityCorrection::WATER),
            _ => Err(TissueHeterogeneityCorrectionError::InvalidValue(
                s.to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dose_units_from_str() {
        assert_eq!(DoseUnits::from_str("GY").unwrap(), DoseUnits::GY);
        assert_eq!(
            DoseUnits::from_str("RELATIVE").unwrap(),
            DoseUnits::RELATIVE
        );
        assert!(matches!(
            DoseUnits::from_str("INVALID"),
            Err(DoseUnitsError::InvalidValue(_))
        ));
    }

    #[test]
    fn test_dose_units_error_message() {
        if let Err(e) = DoseUnits::from_str("INVALID") {
            assert_eq!(e.to_string(), "'INVALID' is not a valid DoseUnits value");
        }
    }

    #[test]
    fn test_dose_summation_type_from_str_plan() {
        assert_eq!(
            DoseSummationType::from_str("PLAN").unwrap(),
            DoseSummationType::PLAN
        );
    }

    #[test]
    fn test_dose_summation_type_from_str_multi_plan() {
        assert_eq!(
            DoseSummationType::from_str("MULTI_PLAN").unwrap(),
            DoseSummationType::MULTI_PLAN
        );
    }

    #[test]
    fn test_dose_summation_type_from_str_plan_overview() {
        assert_eq!(
            DoseSummationType::from_str("PLAN_OVERVIEW").unwrap(),
            DoseSummationType::PLAN_OVERVIEW
        );
    }

    #[test]
    fn test_dose_summation_type_from_str_fraction() {
        assert_eq!(
            DoseSummationType::from_str("FRACTION").unwrap(),
            DoseSummationType::FRACTION
        );
    }

    #[test]
    fn test_dose_summation_type_from_str_beam() {
        assert_eq!(
            DoseSummationType::from_str("BEAM").unwrap(),
            DoseSummationType::BEAM
        );
    }

    #[test]
    fn test_dose_summation_type_from_str_brachy() {
        assert_eq!(
            DoseSummationType::from_str("BRACHY").unwrap(),
            DoseSummationType::BRACHY
        );
    }

    #[test]
    fn test_dose_summation_type_from_str_fraction_session() {
        assert_eq!(
            DoseSummationType::from_str("FRACTION_SESSION").unwrap(),
            DoseSummationType::FRACTION_SESSION
        );
    }

    #[test]
    fn test_dose_summation_type_from_str_beam_session() {
        assert_eq!(
            DoseSummationType::from_str("BEAM_SESSION").unwrap(),
            DoseSummationType::BEAM_SESSION
        );
    }

    #[test]
    fn test_dose_summation_type_from_str_brachy_session() {
        assert_eq!(
            DoseSummationType::from_str("BRACHY_SESSION").unwrap(),
            DoseSummationType::BRACHY_SESSION
        );
    }

    #[test]
    fn test_dose_summation_type_from_str_control_point() {
        assert_eq!(
            DoseSummationType::from_str("CONTROL_POINT").unwrap(),
            DoseSummationType::CONTROL_POINT
        );
    }

    #[test]
    fn test_dose_summation_type_from_str_record() {
        assert_eq!(
            DoseSummationType::from_str("RECORD").unwrap(),
            DoseSummationType::RECORD
        );
    }

    #[test]
    fn test_dose_summation_type_error_message() {
        if let Err(e) = DoseSummationType::from_str("INVALID") {
            assert_eq!(
                e.to_string(),
                "'INVALID' is not a valid DoseSummationType value"
            );
        }
    }

    #[test]
    fn test_dose_summation_type_default() {
        assert_eq!(DoseSummationType::default(), DoseSummationType::NONE);
    }

    #[test]
    fn test_dose_type_from_str_physical() {
        assert_eq!(DoseType::from_str("PHYSICAL").unwrap(), DoseType::PHYSICAL);
    }

    #[test]
    fn test_dose_type_from_str_effective() {
        assert_eq!(
            DoseType::from_str("EFFECTIVE").unwrap(),
            DoseType::EFFECTIVE
        );
    }

    #[test]
    fn test_dose_type_from_str_error() {
        assert_eq!(DoseType::from_str("ERROR").unwrap(), DoseType::ERROR);
    }

    #[test]
    fn test_dose_type_invalid() {
        assert!(matches!(
            DoseType::from_str("INVALID"),
            Err(DoseTypeError::InvalidValue(_))
        ));
    }

    #[test]
    fn test_dose_type_error_message() {
        if let Err(e) = DoseType::from_str("INVALID") {
            assert_eq!(e.to_string(), "'INVALID' is not a valid DoseType value");
        }
    }
    #[test]
    fn test_tissue_heterogeneity_correction_from_str_image() {
        assert_eq!(
            TissueHeterogeneityCorrection::from_str("IMAGE").unwrap(),
            TissueHeterogeneityCorrection::IMAGE
        );
    }

    #[test]
    fn test_tissue_heterogeneity_correction_from_str_roi_override() {
        assert_eq!(
            TissueHeterogeneityCorrection::from_str("ROI_OVERRIDE").unwrap(),
            TissueHeterogeneityCorrection::ROI_OVERRIDE
        );
    }

    #[test]
    fn test_tissue_heterogeneity_correction_from_str_water() {
        assert_eq!(
            TissueHeterogeneityCorrection::from_str("WATER").unwrap(),
            TissueHeterogeneityCorrection::WATER
        );
    }

    #[test]
    fn test_tissue_heterogeneity_correction_from_str_invalid() {
        assert!(matches!(
            TissueHeterogeneityCorrection::from_str("INVALID"),
            Err(TissueHeterogeneityCorrectionError::InvalidValue(_))
        ));
    }

    #[test]
    fn test_tissue_heterogeneity_correction_error_message() {
        if let Err(e) = TissueHeterogeneityCorrection::from_str("INVALID") {
            assert_eq!(
                e.to_string(),
                "'INVALID' is not a valid TissueHeterogeneityCorrection value"
            );
        }
    }
}
