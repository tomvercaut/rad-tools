use crate::{CodeItem, PersonName, PhotometricInterpretation, PixelRepresentation, Sop};
use chrono::{NaiveDate, NaiveDateTime};
use dicom_pixeldata::ndarray::{Array, Ix4};
use std::str::FromStr;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct RTDose {
    pub specific_character_set: String,
    pub instance_creation_dt: Option<NaiveDateTime>,
    pub image_type: Vec<String>,
    pub sop: Sop,
    pub study_dt: Option<NaiveDateTime>,
    pub content_dt: Option<NaiveDateTime>,
    pub accession_number: Option<String>,
    pub ref_physician_name: Option<PersonName>,
    pub station_name: Option<String>,
    pub manufacturer: Option<String>,
    pub referring_physician_name: Option<PersonName>,
    pub manufacturer_model_name: Option<String>,
    pub irradiation_event_uid: String,
    pub patient_name: PersonName,
    pub patient_id: String,
    pub patient_birth_date: Option<NaiveDate>,
    pub patient_sex: String,
    pub patient_identity_removed: bool,
    pub deidentification_method: Option<String>,
    pub slice_thickness: Option<f64>,
    pub software_versions: Option<String>,
    pub study_instance_uid: String,
    pub series_instance_uid: String,
    pub study_id: Option<String>,
    pub series_number: i32,
    pub instance_number: i32,
    pub image_position_patient: [f64; 3],
    pub image_orientation_patient: [f64; 6],
    pub frame_of_reference_uid: String,
    pub position_reference_indicator: Option<String>,
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
    pub dose_units: DoseUnit,
    pub dose_type: DoseType,
    pub dose_comment: Option<String>,
    pub dose_summation_type: DoseSummationType,
    pub grid_frame_offset_vector: Vec<f64>,
    pub dose_grid_scaling: f64,
    pub tissue_heterogeneity_correction: Option<TissueHeterogeneityCorrection>,
    pub referenced_treatment_record_sequence: Option<Vec<ReferencedTreatmentRecord>>,
    pub referenced_rt_plan_sequence: Option<Vec<ReferencedRTPlan>>,
    pub plan_overview_sequence: Option<Vec<PlanOverview>>,
    pub pixel_data_bytes: Vec<u8>,
    pub pixel_data: Array<f64, Ix4>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum DoseUnit {
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

impl FromStr for DoseUnit {
    type Err = DoseUnitsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GY" => Ok(DoseUnit::GY),
            "RELATIVE" => Ok(DoseUnit::RELATIVE),
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

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ReferencedTreatmentRecord {
    pub referenced_sop: Sop,
    pub referenced_beam_sequence: Vec<ReferencedTreatmentRecordReferencedBeam>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ReferencedTreatmentRecordReferencedBeam {
    pub referenced_beam_number: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ReferencedRTPlan {
    pub referenced_sop: Sop,
    pub referenced_fraction_group: Option<Vec<ReferencedFractionGroup>>,
    pub referenced_plan_overview_index: Option<u16>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ReferencedFractionGroup {
    pub referenced_beam_sequence: Vec<ReferencedFractionGroupReferencedBeam>,
    pub referenced_brachy_application_setup_sequence:
        Vec<ReferencedFractionGroupReferencedBrachyApplicationSetup>,
    pub referenced_fraction_group_number: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ReferencedFractionGroupReferencedBeam {
    pub referenced_beam_number: usize,
    pub referenced_control_point_sequence: Vec<ReferencedControlPoint>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ReferencedControlPoint {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ReferencedFractionGroupReferencedBrachyApplicationSetup {
    pub referenced_brachy_application_setup_number: usize,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct PlanOverview {
    pub referenced_image_sequence: Option<Vec<Sop>>,
    pub current_fraction_number: usize,
    pub rt_plan_label: Option<String>,
    pub referenced_structure_set_sequence: Option<Vec<Sop>>,
    pub prescription_overview_sequence: Option<Vec<PrescriptionOverview>>,
    pub plan_overview_index: u16,
    pub number_of_fractions_included: u16,
    pub treatment_site: Option<String>,
    pub treatment_site_code_sequence: Option<Vec<CodeItem>>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct PrescriptionOverview {
    pub referenced_roi_number: usize,
    pub total_prescription_dose: f64,
    pub entity_long_label: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dose_units_from_str() {
        assert_eq!(DoseUnit::from_str("GY").unwrap(), DoseUnit::GY);
        assert_eq!(DoseUnit::from_str("RELATIVE").unwrap(), DoseUnit::RELATIVE);
        assert!(matches!(
            DoseUnit::from_str("INVALID"),
            Err(DoseUnitsError::InvalidValue(_))
        ));
    }

    #[test]
    fn test_dose_units_error_message() {
        if let Err(e) = DoseUnit::from_str("INVALID") {
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
