use crate::{
    ApprovalStatus, BeamDoseType, BeamType, PatientPosition, PersonName, RotationDirection, Sop,
};
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
    pub approval_status: Option<ApprovalStatus>,
    pub review_dt: Option<NaiveDateTime>,
    pub reviewer_name: Option<PersonName>,
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
    pub number_of_brachy_application_setups: i32,
    pub referenced_beam_sequence: Vec<ReferencedBeam>,
    pub referenced_brachy_application_setup_sequence: Option<Vec<ReferencedBrachyApplicationSetup>>,
    pub referenced_dose_reference_sequence: Option<Vec<ReferencedDoseReference>>,
    pub referenced_dose_sequence: Option<Vec<Sop>>,
}

#[derive(Clone, Debug, Default)]
pub struct ReferencedBeam {
    pub referenced_dose_reference_uid: Option<String>,
    pub beam_dose: Option<f64>,
    pub beam_meterset: Option<f64>,
    pub beam_dose_point_depth: Option<f64>,
    pub beam_dose_point_equivalent_depth: Option<f64>,
    pub beam_dose_point_ssd: Option<f64>,
    pub beam_dose_type: Option<BeamDoseType>,
    pub referenced_beam_number: i32,
}

#[derive(Clone, Debug, Default)]
pub struct ReferencedBrachyApplicationSetup {
    pub referenced_dose_reference_uid: Option<String>,
    pub brachy_application_setup_dose_specification_point: Option<Vec<f64>>,
    pub brachy_application_setup_dose: Option<f64>,
    pub referenced_brachy_application_setup_number: i32,
}

#[derive(Clone, Debug, Default)]
pub struct ReferencedDoseReference {
    pub constraint_weight: Option<f64>,
    pub delivery_warning_dose: Option<f64>,
    pub delivery_maximum_dose: Option<f64>,
    pub target_minimum_dose: Option<f64>,
    pub target_prescription_dose: Option<f64>,
    pub target_underdose_volume_fraction: Option<f64>,
    pub organ_at_risk_full_volume_dose: Option<f64>,
    pub organ_at_risk_limit_dose: Option<f64>,
    pub organ_at_risk_maximum_dose: Option<f64>,
    pub organ_at_risk_overdose_volume_fraction: Option<f64>,
    pub referenced_dose_reference_number: Option<i32>,
}

#[derive(Clone, Debug, Default)]
pub struct Beam {
    pub primary_fluence_mode_sequence: Option<Vec<PrimaryFluenceMode>>,
    pub treatment_machine_name: Option<String>,
    pub primary_dosimeter_unit: Option<PrimaryDosimeterUnit>,
    pub source_axis_distance: Option<f64>,
    pub beam_limiting_device_sequence: Vec<BeamLimitingDevice>,
    pub beam_number: i32,
    pub beam_name: Option<String>,
    pub beam_type: Option<BeamType>,
    pub beam_description: Option<String>,
    pub radiation_type: Option<RadiationType>,
    pub treatment_delivery_type: Option<TreatmentDeliveryType>,
    pub number_of_wedges: i32,
    pub number_of_compensators: i32,
    pub number_of_boli: i32,
    pub number_of_blocks: i32,
    pub final_cumulative_meterset_weight: f64,
    pub number_of_control_points: i32,
    pub control_point_sequence: Vec<ControlPoint>,
    pub referenced_patient_setup_number: Option<i32>,
    pub referenced_bolus_sequence: Option<Vec<ReferencedBolus>>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PrimaryFluenceMode {
    pub fluence_mode: FluenceMode,
    pub fluence_mode_id: Option<String>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum FluenceMode {
    #[default]
    None,
    Standard,
    NonStandard,
}

impl FromStr for FluenceMode {
    type Err = FluenceModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "STANDARD" => Ok(FluenceMode::Standard),
            "NON_STANDARD" => Ok(FluenceMode::NonStandard),
            t => Err(FluenceModeError::ParseError(t.into())),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FluenceModeError {
    #[error("Failed to parse fluence mode from: {0}")]
    ParseError(String),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum PrimaryDosimeterUnit {
    #[default]
    None,
    Mu,
    Minute,
}

impl FromStr for PrimaryDosimeterUnit {
    type Err = PrimaryDosimeterUnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "MU" => Ok(PrimaryDosimeterUnit::Mu),
            "MINUTE" => Ok(PrimaryDosimeterUnit::Minute),
            _ => Err(PrimaryDosimeterUnitError::ParseError(s.into())),
        }
    }
}

impl std::fmt::Display for PrimaryDosimeterUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PrimaryDosimeterUnit::Mu => "MU",
            PrimaryDosimeterUnit::Minute => "MINUTE",
            PrimaryDosimeterUnit::None => "NONE",
        };
        write!(f, "{}", s)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PrimaryDosimeterUnitError {
    #[error("Failed to parse primary dosimeter unit from: {0}")]
    ParseError(String),
}

// Define the RadiationType enum
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum RadiationType {
    #[default]
    None,
    Photon,
    Electron,
    Neutron,
    Proton,
}

// Implement the FromStr trait for RadiationType
impl FromStr for RadiationType {
    type Err = RadiationTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PHOTON" => Ok(RadiationType::Photon),
            "ELECTRON" => Ok(RadiationType::Electron),
            "NEUTRON" => Ok(RadiationType::Neutron),
            "PROTON" => Ok(RadiationType::Proton),
            _ => Err(RadiationTypeError::ParseError(s.into())),
        }
    }
}

// Implement the Display trait for RadiationType
impl std::fmt::Display for RadiationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RadiationType::Photon => "PHOTON",
            RadiationType::Electron => "ELECTRON",
            RadiationType::Neutron => "NEUTRON",
            RadiationType::Proton => "PROTON",
            RadiationType::None => "NONE",
        };
        write!(f, "{}", s)
    }
}

// Define a custom error type for RadiationType parsing
#[derive(thiserror::Error, Debug)]
pub enum RadiationTypeError {
    #[error("Failed to parse radiation type from: {0}")]
    ParseError(String),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum TreatmentDeliveryType {
    #[default]
    None,
    Treatment,
    OpenPortFilm,
    TrmtPortFilm,
    Continuation,
    Setup,
}

impl FromStr for TreatmentDeliveryType {
    type Err = TreatmentDeliveryTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TREATMENT" => Ok(TreatmentDeliveryType::Treatment),
            "OPEN_PORTFILM" => Ok(TreatmentDeliveryType::OpenPortFilm),
            "TRMT_PORTFILM" => Ok(TreatmentDeliveryType::TrmtPortFilm),
            "CONTINUATION" => Ok(TreatmentDeliveryType::Continuation),
            "SETUP" => Ok(TreatmentDeliveryType::Setup),
            _ => Err(TreatmentDeliveryTypeError::ParseError(s.into())),
        }
    }
}

impl std::fmt::Display for TreatmentDeliveryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TreatmentDeliveryType::Treatment => "TREATMENT",
            TreatmentDeliveryType::OpenPortFilm => "OPEN_PORTFILM",
            TreatmentDeliveryType::TrmtPortFilm => "TRMT_PORTFILM",
            TreatmentDeliveryType::Continuation => "CONTINUATION",
            TreatmentDeliveryType::Setup => "SETUP",
            TreatmentDeliveryType::None => "NONE",
        };
        write!(f, "{}", s)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum TreatmentDeliveryTypeError {
    #[error("Failed to parse treatment delivery type from: {0}")]
    ParseError(String),
}

#[derive(Clone, Debug, Default)]
pub struct BeamLimitingDevice {
    pub rt_beam_limiting_device_type: RTBeamLimitingDeviceType,
    pub source_to_beam_limiting_device_distance: Option<f64>,
    pub number_of_leaf_jaw_pairs: i32,
    pub leaf_position_boundaries: Option<Vec<f64>>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum RTBeamLimitingDeviceType {
    #[default]
    None,
    X,
    Y,
    AsymX,
    AsymY,
    MlcX,
    MlcY,
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
            "ASYMX" => Ok(RTBeamLimitingDeviceType::AsymX),
            "ASYMY" => Ok(RTBeamLimitingDeviceType::AsymY),
            "MLCX" => Ok(RTBeamLimitingDeviceType::MlcX),
            "MLCY" => Ok(RTBeamLimitingDeviceType::MlcY),
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
    pub nominal_beam_energy: Option<f64>,
    pub beam_limiting_device_position_sequence: Option<Vec<BeamLimitingDevicePosition>>,
    pub gantry_angle: Option<f64>,
    pub gantry_rotation_direction: Option<RotationDirection>,
    pub beam_limiting_device_angle: Option<f64>,
    pub beam_limiting_device_rotation_direction: Option<RotationDirection>,
    pub patient_support_angle: Option<f64>,
    pub patient_support_rotation_direction: Option<RotationDirection>,
    pub table_top_eccentric_angle: Option<f64>,
    pub table_top_eccentric_rotation_direction: Option<RotationDirection>,
    pub table_top_vertical_position: Option<f64>,
    pub table_top_longitudinal_position: Option<f64>,
    pub table_top_lateral_position: Option<f64>,
    pub isocenter_position: Option<Vec<f64>>,
    pub source_to_surface_distance: Option<f64>,
    pub cumulative_meterset_weight: Option<f64>,
    pub table_top_pitch_angle: Option<f64>,
    pub table_top_pitch_rotation_direction: Option<RotationDirection>,
    pub table_top_roll_angle: Option<f64>,
    pub table_top_roll_rotation_direction: Option<RotationDirection>,
    pub gantry_pitch_angle: Option<f64>,
    pub gantry_pitch_rotation_direction: Option<RotationDirection>,
}

#[derive(Clone, Debug, Default)]
pub struct PatientSetup {
    pub patient_position: PatientPosition,
    pub patient_setup_number: i32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReferencedBolus {
    pub referenced_roi_number: i32,
    pub bolus_id: Option<String>,
    pub bolus_description: Option<String>,
    pub accessory_code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fluence_mode_from_str_standard() {
        let mode: FluenceMode = "STANDARD".parse().unwrap();
        assert_eq!(mode, FluenceMode::Standard);
    }

    #[test]
    fn test_fluence_mode_from_str_non_standard() {
        let mode: FluenceMode = "NON_STANDARD".parse().unwrap();
        assert_eq!(mode, FluenceMode::NonStandard);
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
        assert_eq!(device_type, RTBeamLimitingDeviceType::AsymX);
    }

    #[test]
    fn test_beam_limiting_device_type_from_str_asymy() {
        let device_type: RTBeamLimitingDeviceType = "ASYMY".parse().unwrap();
        assert_eq!(device_type, RTBeamLimitingDeviceType::AsymY);
    }

    #[test]
    fn test_beam_limiting_device_type_from_str_mlxcx() {
        let device_type: RTBeamLimitingDeviceType = "MLCX".parse().unwrap();
        assert_eq!(device_type, RTBeamLimitingDeviceType::MlcX);
    }

    #[test]
    fn test_beam_limiting_device_type_from_str_mlcy() {
        let device_type: RTBeamLimitingDeviceType = "MLCY".parse().unwrap();
        assert_eq!(device_type, RTBeamLimitingDeviceType::MlcY);
    }

    #[test]
    fn test_beam_limiting_device_type_from_str_invalid() {
        let device_type_result: Result<RTBeamLimitingDeviceType, RTBeamLimitingDeviceTypeError> =
            "INVALID".parse();
        assert!(device_type_result.is_err());
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            PrimaryDosimeterUnit::from_str("MU").unwrap(),
            PrimaryDosimeterUnit::Mu
        );
        assert_eq!(
            PrimaryDosimeterUnit::from_str("minute").unwrap(),
            PrimaryDosimeterUnit::Minute
        );
        assert!(PrimaryDosimeterUnit::from_str("unknown").is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(PrimaryDosimeterUnit::Mu.to_string(), "MU");
        assert_eq!(PrimaryDosimeterUnit::Minute.to_string(), "MINUTE");
    }

    #[test]
    fn test_radiation_type_from_str() {
        assert_eq!(
            RadiationType::from_str("PHOTON").unwrap(),
            RadiationType::Photon
        );
        assert_eq!(
            RadiationType::from_str("ELECTRON").unwrap(),
            RadiationType::Electron
        );
        assert_eq!(
            RadiationType::from_str("NEUTRON").unwrap(),
            RadiationType::Neutron
        );
        assert_eq!(
            RadiationType::from_str("PROTON").unwrap(),
            RadiationType::Proton
        );
        assert!(RadiationType::from_str("UNKNOWN").is_err());
    }

    #[test]
    fn test_radiation_type_display() {
        assert_eq!(RadiationType::Photon.to_string(), "PHOTON");
        assert_eq!(RadiationType::Electron.to_string(), "ELECTRON");
        assert_eq!(RadiationType::Neutron.to_string(), "NEUTRON");
        assert_eq!(RadiationType::Proton.to_string(), "PROTON");
    }

    #[test]
    fn test_treatment_delivery_type_from_str() {
        assert_eq!(
            TreatmentDeliveryType::from_str("TREATMENT").unwrap(),
            TreatmentDeliveryType::Treatment
        );
        assert_eq!(
            TreatmentDeliveryType::from_str("OPEN_PORTFILM").unwrap(),
            TreatmentDeliveryType::OpenPortFilm
        );
        assert_eq!(
            TreatmentDeliveryType::from_str("TRMT_PORTFILM").unwrap(),
            TreatmentDeliveryType::TrmtPortFilm
        );
        assert_eq!(
            TreatmentDeliveryType::from_str("CONTINUATION").unwrap(),
            TreatmentDeliveryType::Continuation
        );
        assert_eq!(
            TreatmentDeliveryType::from_str("SETUP").unwrap(),
            TreatmentDeliveryType::Setup
        );
        assert!(TreatmentDeliveryType::from_str("INVALID").is_err());
    }

    #[test]
    fn test_treatment_delivery_type_display() {
        assert_eq!(format!("{}", TreatmentDeliveryType::Treatment), "TREATMENT");
        assert_eq!(
            format!("{}", TreatmentDeliveryType::OpenPortFilm),
            "OPEN_PORTFILM"
        );
        assert_eq!(
            format!("{}", TreatmentDeliveryType::TrmtPortFilm),
            "TRMT_PORTFILM"
        );
        assert_eq!(
            format!("{}", TreatmentDeliveryType::Continuation),
            "CONTINUATION"
        );
        assert_eq!(format!("{}", TreatmentDeliveryType::Setup), "SETUP");
    }
}
