use crate::{ApprovalStatus, PatientPosition, PersonName, RotationDirection, Sop};
use chrono::{NaiveDate, NaiveDateTime};
use std::str::FromStr;

#[derive(Clone, Debug, Default)]
pub struct RTPlan {
    pub specific_character_set: String,
    pub instance_creation_dt: Option<NaiveDateTime>,
    pub sop: Sop,
    pub study_dt: Option<NaiveDateTime>,
    pub accession_number: Option<String>,
    pub manufacturer: Option<String>,
    pub referring_physician_name: Option<PersonName>,
    pub manufacturer_model_name: Option<String>,
    pub patient_name: PersonName,
    pub patient_id: String,
    pub patient_birth_date: Option<NaiveDate>,
    pub patient_sex: String,
    pub patient_identity_removed: bool,
    pub deidentification_method: Option<String>,
    pub software_versions: Option<String>,
    pub study_instance_uid: String,
    pub series_instance_uid: String,
    pub study_id: Option<String>,
    pub series_number: Option<i32>,
    pub frame_of_reference_uid: String,
    pub position_reference_indicator: Option<String>,
    pub rt_plan_label: String,
    pub rt_plan_name: Option<String>,
    pub rt_plan_description: Option<String>,
    pub rt_plan_dt: Option<NaiveDateTime>,
    pub treatment_protocols: Option<String>,
    pub plan_intent: Option<String>,
    pub rt_plan_geometry: String,
    pub fraction_group_sequence: Vec<FractionGroup>,
    pub beam_sequence: Vec<Beam>,
    pub patient_setup_sequence: Vec<PatientSetup>,
    pub referenced_structure_set_sequence: Vec<Sop>,
    pub approval_status: ApprovalStatus,
    pub review_dt: Option<NaiveDateTime>,
    pub reviewer_name: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct FractionGroup {
    pub fraction_group_number: i32,
    pub fraction_group_description: Option<String>,
    pub number_of_fractions_planned: Option<i32>,
    pub number_of_fraction_pattern_digits_per_day: Option<i32>,
    pub repeat_fraction_cycle_length: Option<i32>,
    pub fraction_pattern: Option<String>,
    pub number_of_beams: i32,
    pub beam_dose_meaning: Option<String>,
    pub number_of_brachy_application_sequences: i32,
    pub referenced_beam_sequence: Vec<ReferencedBeam>,
    pub referenced_brachy_application_setup_sequence: Vec<ReferencedBrachyApplicationSetup>,
    pub referenced_dose_reference_sequence: Option<Vec<ReferencedDoseReference>>,
    pub referenced_dose_sequence: Option<Vec<Sop>>,
}

#[derive(Clone, Debug, Default)]
pub struct ReferencedBeam {
    pub beam_dose_specification_point: Vec<f64>,
    pub beam_dose: f64,
    pub beam_meterset: f64,
    pub beam_dose_point_depth: f64,
    pub beam_dose_point_equivalent_depth: f64,
    pub beam_dose_point_ssd: f64,
    pub beam_dose_type: String,
    pub referenced_beam_number: i32,
}

#[derive(Clone, Debug, Default)]
pub struct ReferencedBrachyApplicationSetup {
    pub referenced_dose_reference_uid: String,
    pub brachy_application_setup_dose_specification_point: Vec<f64>,
    pub brachy_application_setup_dose: f64,
    pub referenced_brachy_application_setup_number: i32,
}

#[derive(Clone, Debug, Default)]
pub struct ReferencedDoseReference {
    pub constraint_weight: f64,
    pub delivery_warning_dose: f64,
    pub delivery_maximum_dose: f64,
    pub target_minimum_dose: f64,
    pub target_prescription_dose: f64,
    pub target_underdose_volume_fraction: f64,
    pub organ_at_risk_full_volume_dose: f64,
    pub organ_at_risk_full_limit_dose: f64,
    pub organ_at_risk_full_maximum_dose: f64,
    pub organ_at_risk_full_overdose_volume_fraction: f64,
    pub referenced_dose_reference_number: i32,
}

#[derive(Clone, Debug, Default)]
pub struct Beam {
    pub primary_fluence_mode_sequence: Vec<PrimaryFluenceMode>,
    pub treatment_machine_name: String,
    pub primary_dosimeter_unit: String,
    pub source_axis_distance: f64,
    pub beam_limiting_device_sequence: Vec<BeamLimitingDevice>,
    pub beam_number: i32,
    pub beam_name: String,
    pub beam_description: String,
    pub radiation_type: String,
    pub treatment_delivery_type: String,
    pub number_of_wedges: i32,
    pub number_of_compensators: i32,
    pub number_of_boli: i32,
    pub number_of_blocks: i32,
    pub final_cumulative_meterset_weight: f64,
    pub number_of_control_points: i32,
    pub control_point_sequence: Vec<ControlPoint>,
    pub referenced_patient_setup_number: i32,
}

#[derive(Clone, Debug, Default)]
pub struct PrimaryFluenceMode {
    pub fluence_mode: FluenceMode,
    pub fluence_mode_id: String,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum FluenceMode {
    #[default]
    NONE,
    STANDARD,
    NON_STANDARD,
}

impl FromStr for FluenceMode {
    type Err = FluenceModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "STANDARD" => Ok(FluenceMode::STANDARD),
            "NON_STANDARD" => Ok(FluenceMode::NON_STANDARD),
            t => Err(FluenceModeError::ParseError(t.into())),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FluenceModeError {
    #[error("Failed to parse fluence mode from: {0}")]
    ParseError(String),
}

#[derive(Clone, Debug, Default)]
pub struct BeamLimitingDevice {
    pub rt_beam_limiting_device_type: RTBeamLimitingDeviceType,
    pub number_of_leaf_jaw_pairs: i32,
    pub leaf_position_boundaries: Vec<f64>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum RTBeamLimitingDeviceType {
    #[default]
    NONE,
    X,
    Y,
    ASYMX,
    ASYMY,
    MLCX,
    MLCY,
}

#[derive(thiserror::Error, Debug)]
pub enum RTBeamLimitingDeviceTypeError {
    #[error("Failed to parse RT Beam Limiting Device Type from: {0}")]
    ParseError(String),
}

impl FromStr for RTBeamLimitingDeviceType {
    type Err = RTBeamLimitingDeviceTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "X" => Ok(RTBeamLimitingDeviceType::X),
            "Y" => Ok(RTBeamLimitingDeviceType::Y),
            "ASYMX" => Ok(RTBeamLimitingDeviceType::ASYMX),
            "ASYMY" => Ok(RTBeamLimitingDeviceType::ASYMY),
            "MLCX" => Ok(RTBeamLimitingDeviceType::MLCX),
            "MLCY" => Ok(RTBeamLimitingDeviceType::MLCY),
            t => Err(RTBeamLimitingDeviceTypeError::ParseError(t.into())),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct BeamLimitingDevicePosition {
    pub rt_beam_limiting_device_type: RTBeamLimitingDeviceType,
    pub leaf_jaw_positions: Vec<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct ControlPoint {
    pub control_point_index: i32,
    pub nominal_beam_energy: f64,
    pub beam_limiting_device_position_sequence: Vec<BeamLimitingDevicePosition>,
    pub gantry_angle: f64,
    pub gantry_rotation_direction: RotationDirection,
    pub beam_limiting_device_angle: f64,
    pub beam_limiting_device_rotation_direction: RotationDirection,
    pub patient_support_angle: f64,
    pub patient_support_rotation_direction: RotationDirection,
    pub table_top_eccentric_angle: f64,
    pub table_top_eccentric_rotation_direction: RotationDirection,
    pub table_top_vertical_position: f64,
    pub table_top_longitudinal_position: f64,
    pub table_top_lateral_position: f64,
    pub isocenter_position: Vec<f64>,
    pub source_to_surface_distance: f64,
    pub cumulative_meterset_weight: f64,
    pub table_top_pitch_angle: f64,
    pub table_top_pitch_rotation_direction: RotationDirection,
    pub table_top_roll_angle: f64,
    pub table_top_roll_rotation_direction: RotationDirection,
    pub gantry_pitch_angle: f64,
    pub gantry_pitch_rotation_direction: RotationDirection,
}

#[derive(Clone, Debug, Default)]
pub struct PatientSetup {
    pub patient_position: PatientPosition,
    pub patient_setup_number: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fluence_mode_from_str_standard() {
        let mode: FluenceMode = "STANDARD".parse().unwrap();
        assert_eq!(mode, FluenceMode::STANDARD);
    }

    #[test]
    fn test_fluence_mode_from_str_non_standard() {
        let mode: FluenceMode = "NON_STANDARD".parse().unwrap();
        assert_eq!(mode, FluenceMode::NON_STANDARD);
    }

    #[test]
    fn test_fluence_mode_from_str_invalid() {
        let mode_result: Result<FluenceMode, FluenceModeError> = "INVALID".parse();
        assert!(mode_result.is_err());
    }

    #[test]
    fn test_beam_limiting_device_type_from_str_x() {
        let device_type: RTBeamLimitingDeviceType = "X".parse().unwrap();
        assert_eq!(device_type, RTBeamLimitingDeviceType::X);
    }

    #[test]
    fn test_beam_limiting_device_type_from_str_y() {
        let device_type: RTBeamLimitingDeviceType = "Y".parse().unwrap();
        assert_eq!(device_type, RTBeamLimitingDeviceType::Y);
    }

    #[test]
    fn test_beam_limiting_device_type_from_str_asymx() {
        let device_type: RTBeamLimitingDeviceType = "ASYMX".parse().unwrap();
        assert_eq!(device_type, RTBeamLimitingDeviceType::ASYMX);
    }

    #[test]
    fn test_beam_limiting_device_type_from_str_asymy() {
        let device_type: RTBeamLimitingDeviceType = "ASYMY".parse().unwrap();
        assert_eq!(device_type, RTBeamLimitingDeviceType::ASYMY);
    }

    #[test]
    fn test_beam_limiting_device_type_from_str_mlxcx() {
        let device_type: RTBeamLimitingDeviceType = "MLCX".parse().unwrap();
        assert_eq!(device_type, RTBeamLimitingDeviceType::MLCX);
    }

    #[test]
    fn test_beam_limiting_device_type_from_str_mlcy() {
        let device_type: RTBeamLimitingDeviceType = "MLCY".parse().unwrap();
        assert_eq!(device_type, RTBeamLimitingDeviceType::MLCY);
    }

    #[test]
    fn test_beam_limiting_device_type_from_str_invalid() {
        let device_type_result: Result<RTBeamLimitingDeviceType, RTBeamLimitingDeviceTypeError> =
            "INVALID".parse();
        assert!(device_type_result.is_err());
    }
}
