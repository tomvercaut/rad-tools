use std::str::FromStr;

#[derive(Clone, Debug, Default)]
pub struct Sop {
    pub class_uid: String,
    pub instance_uid: String,
}

#[derive(Clone, Debug, Default)]
pub struct PersonName {
    pub family_name: String,
    pub given_name: String,
    pub middle_name: String,
    pub prefix: String,
    pub suffix: String,
}

impl FromStr for PersonName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split('^').map(|x| x.trim()).collect();
        let n = v.len();
        let mut name = PersonName::default();
        if n > 0 {
            name.family_name = v[0].to_string();
        }
        if n > 1 {
            name.given_name = v[1].to_string();
        }
        if n > 2 {
            name.middle_name = v[2].to_string();
        }
        if n > 3 {
            name.prefix = v[3].to_string();
        }
        if n > 4 {
            name.suffix = v[4].to_string();
        }
        Ok(name)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum RotationDirectionError {
    #[error("Invalid rotation direction: {0}")]
    InvalidRotationDirection(String),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum RotationDirection {
    #[default]
    NONE,
    CW,
    CCW,
}

impl FromStr for RotationDirection {
    type Err = RotationDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "CW" || s == "cw" {
            Ok(RotationDirection::CW)
        } else if s == "CCW" || s == "ccw" {
            Ok(RotationDirection::CCW)
        } else if s == "None" || s == "none" || s == "NONE" {
            Ok(RotationDirection::NONE)
        } else {
            Err(RotationDirectionError::InvalidRotationDirection(s.into()))
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PatientPositionError {
    #[error("Invalid patient position: {0}")]
    InvalidPatientPosition(String),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum PatientPosition {
    #[default]
    NONE, // Undefined default
    HFP,  // Head First-Prone
    HFS,  // Head First-Supine
    HFDR, // Head First-Decubitus Right
    HFDL, // Head First-Decubitus Left
    FFDR, // Feet First-Decubitus Right
    FFDL, // Feet First-Decubitus Left
    FFP,  // Feet First-Prone
    FFS,  // Feet First-Supine
}

impl FromStr for PatientPosition {
    type Err = PatientPositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HFP" => Ok(PatientPosition::HFP),
            "HFS" => Ok(PatientPosition::HFS),
            "HFDR" => Ok(PatientPosition::HFDR),
            "HFDL" => Ok(PatientPosition::HFDL),
            "FFDR" => Ok(PatientPosition::FFDR),
            "FFDL" => Ok(PatientPosition::FFDL),
            "FFP" => Ok(PatientPosition::FFP),
            "FFS" => Ok(PatientPosition::FFS),
            _ => Err(PatientPositionError::InvalidPatientPosition(s.to_string())),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CodeItem {
    pub code_value: String,
    pub coding_scheme_designator: String,
    pub code_meaning: String,
}

#[derive(thiserror::Error, Debug)]
pub enum PhotometricInterpretationError {
    #[error("Invalid photometric interpretation: {0}")]
    InvalidPhotometricInterpretation(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[allow(non_camel_case_types)]
pub enum PhotometricInterpretation {
    MONOCHROME1, // Monochrome 1
    #[default]
    MONOCHROME2, // Monochrome 2
    PALETTE_COLOR, // Palette Color
    RGB,         // RGB
    YBR_FULL,    // YBR_FULL
    YBR_FULL_422, // YBR_FULL_422
    YBR_PARTIAL_422, // YBR_PARTIAL_422
    YBR_PARTIAL_420, // YBR_PARTIAL_420
    YBR_ICT,     // YBR_ICT
    YBR_RCT,     // YBR_RCT
}

impl FromStr for PhotometricInterpretation {
    type Err = PhotometricInterpretationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MONOCHROME1" => Ok(PhotometricInterpretation::MONOCHROME1),
            "MONOCHROME2" => Ok(PhotometricInterpretation::MONOCHROME2),
            "PALETTE_COLOR" => Ok(PhotometricInterpretation::PALETTE_COLOR),
            "RGB" => Ok(PhotometricInterpretation::RGB),
            "YBR_FULL" => Ok(PhotometricInterpretation::YBR_FULL),
            "YBR_FULL_422" => Ok(PhotometricInterpretation::YBR_FULL_422),
            "YBR_PARTIAL_422" => Ok(PhotometricInterpretation::YBR_PARTIAL_422),
            "YBR_PARTIAL_420" => Ok(PhotometricInterpretation::YBR_PARTIAL_420),
            "YBR_ICT" => Ok(PhotometricInterpretation::YBR_ICT),
            "YBR_RCT" => Ok(PhotometricInterpretation::YBR_RCT),
            _ => {
                Err(PhotometricInterpretationError::InvalidPhotometricInterpretation(s.to_string()))
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PixelRepresentationError {
    #[error("Invalid pixel representation: {0}")]
    InvalidPixelRepresentation(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum PixelRepresentation {
    #[default]
    Unsigned, // Unsigned integer
    Signed, // Signed integer (2's complement)
}

impl FromStr for PixelRepresentation {
    type Err = PixelRepresentationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(PixelRepresentation::Unsigned),
            "1" => Ok(PixelRepresentation::Signed),
            _ => Err(PixelRepresentationError::InvalidPixelRepresentation(
                s.to_string(),
            )),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RescaleTypeError {
    #[error("Invalid rescale type: {0}")]
    InvalidRescaleType(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[allow(non_camel_case_types)]
pub enum RescaleType {
    #[default]
    US, // Unspecified
    OD,     // Optical Density
    HU,     // Hounsfield Units
    MGML,   // Milligram per Milliliter
    Z_EFF,  // Effective Atomic Number
    ED,     // Electron Density
    EDW,    // Electron Density Weighted
    HU_MOD, // Modified Hounsfield Units
    PCT,    // Percent
}

impl FromStr for RescaleType {
    type Err = RescaleTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "US" => Ok(RescaleType::US),
            "OD" => Ok(RescaleType::OD),
            "HU" => Ok(RescaleType::HU),
            "MGML" => Ok(RescaleType::MGML),
            "Z_EFF" => Ok(RescaleType::Z_EFF),
            "ED" => Ok(RescaleType::ED),
            "EDW" => Ok(RescaleType::EDW),
            "HU_MOD" => Ok(RescaleType::HU_MOD),
            "PCT" => Ok(RescaleType::PCT),
            _ => Err(RescaleTypeError::InvalidRescaleType(s.to_string())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ApprovalStatus {
    APPROVED,
    #[default]
    UNAPPROVED,
    REJECTED,
}

impl FromStr for ApprovalStatus {
    type Err = ApprovalStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "APPROVED" => Ok(ApprovalStatus::APPROVED),
            "UNAPPROVED" => Ok(ApprovalStatus::UNAPPROVED),
            "REJECTED" => Ok(ApprovalStatus::REJECTED),
            _ => Err(ApprovalStatusError::InvalidApprovalStatus(s.into())),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ApprovalStatusError {
    #[error("Invalid approval status: {0}")]
    InvalidApprovalStatus(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str_complete() {
        let input = "Smith^John^Doe^Mr.^Jr.";
        let name = PersonName::from_str(input).unwrap();
        assert_eq!(name.family_name, "Smith");
        assert_eq!(name.given_name, "John");
        assert_eq!(name.middle_name, "Doe");
        assert_eq!(name.prefix, "Mr.");
        assert_eq!(name.suffix, "Jr.");
    }

    #[test]
    fn test_from_str_partial1() {
        let input = "Smith^John";
        let name = PersonName::from_str(input).unwrap();
        assert_eq!(name.family_name, "Smith");
        assert_eq!(name.given_name, "John");
        assert!(name.middle_name.is_empty());
        assert!(name.prefix.is_empty());
        assert!(name.suffix.is_empty());
    }

    #[test]
    fn test_from_str_partial2() {
        let input = "Smith^John^Doe";
        let name = PersonName::from_str(input).unwrap();
        assert_eq!(name.family_name, "Smith");
        assert_eq!(name.given_name, "John");
        assert_eq!(name.middle_name, "Doe");
        assert!(name.prefix.is_empty());
        assert!(name.suffix.is_empty());
    }

    #[test]
    fn test_from_str_partial3() {
        let input = "Smith^John^Doe^^Jr";
        let name = PersonName::from_str(input).unwrap();
        assert_eq!(name.family_name, "Smith");
        assert_eq!(name.given_name, "John");
        assert_eq!(name.middle_name, "Doe");
        assert!(name.prefix.is_empty());
        assert_eq!(name.suffix, "Jr");
    }

    #[test]
    fn test_from_str_empty() {
        let input = "";
        let name = PersonName::from_str(input).unwrap();
        assert!(name.family_name.is_empty());
        assert!(name.given_name.is_empty());
        assert!(name.middle_name.is_empty());
        assert!(name.prefix.is_empty());
        assert!(name.suffix.is_empty());
    }

    #[test]
    fn test_from_str_with_whitespace() {
        let input = " Smith ^ John ^ Doe ^ Mr. ^ Jr. ";
        let name = PersonName::from_str(input).unwrap();
        assert_eq!(name.family_name, "Smith");
        assert_eq!(name.given_name, "John");
        assert_eq!(name.middle_name, "Doe");
        assert_eq!(name.prefix, "Mr.");
        assert_eq!(name.suffix, "Jr.");
    }

    #[test]
    fn test_rotation_direction_from_str_none() {
        let input = "NONE";
        let direction = RotationDirection::from_str(input).unwrap();
        assert_eq!(direction, RotationDirection::NONE);

        let input = "none";
        let direction = RotationDirection::from_str(input).unwrap();
        assert_eq!(direction, RotationDirection::NONE);

        let input = "None";
        let direction = RotationDirection::from_str(input).unwrap();
        assert_eq!(direction, RotationDirection::NONE);
    }

    #[test]
    fn test_rotation_direction_from_str_cw() {
        let input = "CW";
        let direction = RotationDirection::from_str(input).unwrap();
        assert_eq!(direction, RotationDirection::CW);

        let input = "cw";
        let direction = RotationDirection::from_str(input).unwrap();
        assert_eq!(direction, RotationDirection::CW);
    }

    #[test]
    fn test_rotation_direction_from_str_ccw() {
        let input = "CCW";
        let direction = RotationDirection::from_str(input).unwrap();
        assert_eq!(direction, RotationDirection::CCW);

        let input = "ccw";
        let direction = RotationDirection::from_str(input).unwrap();
        assert_eq!(direction, RotationDirection::CCW);
    }

    #[test]
    fn test_rotation_direction_from_str_invalid() {
        let input = "INVALID";
        let direction = RotationDirection::from_str(input);
        assert!(direction.is_err());
    }

    #[test]
    fn test_patient_position_from_str_hfp() {
        let input = "HFP";
        let position = PatientPosition::from_str(input).unwrap();
        assert_eq!(position, PatientPosition::HFP);
    }

    #[test]
    fn test_patient_position_from_str_hfs() {
        let input = "HFS";
        let position = PatientPosition::from_str(input).unwrap();
        assert_eq!(position, PatientPosition::HFS);
    }

    #[test]
    fn test_patient_position_from_str_hfdr() {
        let input = "HFDR";
        let position = PatientPosition::from_str(input).unwrap();
        assert_eq!(position, PatientPosition::HFDR);
    }

    #[test]
    fn test_patient_position_from_str_hfdl() {
        let input = "HFDL";
        let position = PatientPosition::from_str(input).unwrap();
        assert_eq!(position, PatientPosition::HFDL);
    }

    #[test]
    fn test_patient_position_from_str_ffdr() {
        let input = "FFDR";
        let position = PatientPosition::from_str(input).unwrap();
        assert_eq!(position, PatientPosition::FFDR);
    }

    #[test]
    fn test_patient_position_from_str_ffdl() {
        let input = "FFDL";
        let position = PatientPosition::from_str(input).unwrap();
        assert_eq!(position, PatientPosition::FFDL);
    }

    #[test]
    fn test_patient_position_from_str_ffp() {
        let input = "FFP";
        let position = PatientPosition::from_str(input).unwrap();
        assert_eq!(position, PatientPosition::FFP);
    }

    #[test]
    fn test_patient_position_from_str_ffs() {
        let input = "FFS";
        let position = PatientPosition::from_str(input).unwrap();
        assert_eq!(position, PatientPosition::FFS);
    }

    #[test]
    fn test_patient_position_from_str_invalid() {
        let input = "INVALID";
        let position = PatientPosition::from_str(input);
        assert!(position.is_err());
        if let Err(PatientPositionError::InvalidPatientPosition(pos)) = position {
            assert_eq!(pos, "INVALID".to_string());
        } else {
            panic!("Expected InvalidPatientPosition error");
        }
    }

    #[test]
    fn test_photometric_interpretation_from_str_monochrome1() {
        let input = "MONOCHROME1";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::MONOCHROME1);
    }

    #[test]
    fn test_photometric_interpretation_from_str_monochrome2() {
        let input = "MONOCHROME2";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::MONOCHROME2);
    }

    #[test]
    fn test_photometric_interpretation_from_str_palcolor() {
        let input = "PALETTE_COLOR";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::PALETTE_COLOR);
    }

    #[test]
    fn test_photometric_interpretation_from_str_rgb() {
        let input = "RGB";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::RGB);
    }

    #[test]
    fn test_photometric_interpretation_from_str_ybr_full() {
        let input = "YBR_FULL";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::YBR_FULL);
    }

    #[test]
    fn test_photometric_interpretation_from_str_ybr_full_422() {
        let input = "YBR_FULL_422";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::YBR_FULL_422);
    }

    #[test]
    fn test_photometric_interpretation_from_str_ybr_partial_422() {
        let input = "YBR_PARTIAL_422";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::YBR_PARTIAL_422);
    }

    #[test]
    fn test_photometric_interpretation_from_str_ybr_rct() {
        let input = "YBR_RCT";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::YBR_RCT);
    }

    #[test]
    fn test_photometric_interpretation_from_str_ybr_ict() {
        let input = "YBR_ICT";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::YBR_ICT);
    }

    #[test]
    fn test_photometric_interpretation_from_str_invalid() {
        let input = "INVALID";
        let interpretation = PhotometricInterpretation::from_str(input);
        assert!(interpretation.is_err());
        if let Err(PhotometricInterpretationError::InvalidPhotometricInterpretation(pos)) =
            interpretation
        {
            assert_eq!(pos, "INVALID".to_string());
        } else {
            panic!("Expected InvalidPhotometricInterpretation error");
        }
    }

    #[test]
    fn test_pixel_representation_from_str() {
        assert_eq!(
            PixelRepresentation::from_str("0").unwrap(),
            PixelRepresentation::Unsigned
        );
        assert_eq!(
            PixelRepresentation::from_str("1").unwrap(),
            PixelRepresentation::Signed
        );
        assert!(PixelRepresentation::from_str("2").is_err());
    }

    #[test]
    fn test_rescale_type_from_str_us() {
        let input = "US";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::US);
    }

    #[test]
    fn test_rescale_type_from_str_od() {
        let input = "OD";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::OD);
    }

    #[test]
    fn test_rescale_type_from_str_hu() {
        let input = "HU";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::HU);
    }

    #[test]
    fn test_rescale_type_from_str_mgml() {
        let input = "MGML";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::MGML);
    }

    #[test]
    fn test_rescale_type_from_str_z_eff() {
        let input = "Z_EFF";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::Z_EFF);
    }

    #[test]
    fn test_rescale_type_from_str_ed() {
        let input = "ED";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::ED);
    }

    #[test]
    fn test_rescale_type_from_str_edw() {
        let input = "EDW";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::EDW);
    }

    #[test]
    fn test_rescale_type_from_str_hu_mod() {
        let input = "HU_MOD";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::HU_MOD);
    }

    #[test]
    fn test_rescale_type_from_str_pct() {
        let input = "PCT";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::PCT);
    }

    #[test]
    fn test_rescale_type_from_str_invalid() {
        let input = "INVALID";
        let rescale_type_result = RescaleType::from_str(input);
        assert!(rescale_type_result.is_err());
    }

    #[test]
    fn test_approval_status_from_str_approved() {
        let input = "Approved";
        let status = ApprovalStatus::from_str(input).unwrap();
        assert_eq!(status, ApprovalStatus::APPROVED);
    }

    #[test]
    fn test_approval_status_from_str_pending() {
        let input = "UNAPPROVED";
        let status = ApprovalStatus::from_str(input).unwrap();
        assert_eq!(status, ApprovalStatus::UNAPPROVED);
    }

    #[test]
    fn test_approval_status_from_str_rejected() {
        let input = "Rejected";
        let status = ApprovalStatus::from_str(input).unwrap();
        assert_eq!(status, ApprovalStatus::REJECTED);
    }

    #[test]
    fn test_approval_status_from_str_invalid() {
        let input = "INVALID";
        let status_result = ApprovalStatus::from_str(input);
        assert!(status_result.is_err());
    }
}
