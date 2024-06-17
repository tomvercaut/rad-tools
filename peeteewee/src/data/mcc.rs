use std::str::FromStr;

use crate::PeeTeeWeeError;
use crate::PeeTeeWeeError::{
    InvalidStrToCurveType, InvalidStrToOrientation, InvalidStrToRotationDirection,
};

#[derive(Clone, Debug, Default)]
pub struct Mcc {
    pub format: String,
    pub file_creation_date: String,
    pub last_modified: String,
    pub scans: Vec<Scan>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Rotation {
    None,
    CW,
    CC,
}

impl Default for Rotation {
    fn default() -> Self {
        Self::None
    }
}

impl FromStr for Rotation {
    type Err = PeeTeeWeeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "CW" {
            Ok(Self::CW)
        } else if s == "CC" {
            Ok(Self::CC)
        } else if s == "NONE" {
            Ok(Self::None)
        } else {
            Err(InvalidStrToRotationDirection(s.to_string()))
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Orientation {
    None,
    Horizontal,
    Vertical,
}

impl FromStr for Orientation {
    type Err = PeeTeeWeeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "HORIZONTAL" {
            Ok(Self::Horizontal)
        } else if s == "VERTICAL" {
            Ok(Self::Vertical)
        } else if s == "NONE" {
            Ok(Self::None)
        } else {
            Err(InvalidStrToOrientation(s.to_string()))
        }
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CurveType {
    None,
    InplaneProfile,
    CrossplaneProfile,
}

impl FromStr for CurveType {
    type Err = PeeTeeWeeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "INPLANE_PROFILE" {
            Ok(Self::InplaneProfile)
        } else if s == "CROSSPLANE_PROFILE" {
            Ok(Self::CrossplaneProfile)
        } else if s == "NONE" {
            Ok(Self::None)
        } else {
            Err(InvalidStrToCurveType(s.to_string()))
        }
    }
}

impl Default for CurveType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug, Default)]
pub struct Scan {
    pub task_name: String,
    pub program: String,
    pub meas_date: String,
    pub linac: String,
    pub modality: String,
    pub isocenter: f64,
    pub inplane_axis: String,
    pub crossplane_axis: String,
    pub depth_axis: String,
    pub inplane_axis_dir: String,
    pub crossplane_axis_dir: String,
    pub depth_axis_dir: String,
    pub energy: f64,
    pub nominal_dmax: f64,
    pub ssd: f64,
    pub block: bool,
    pub wedge_angle: f64,
    pub field_inplane: f64,
    pub field_crossplane: f64,
    pub field_type: String,
    pub gantry: f64,
    pub gantry_upright_position: bool,
    pub gantry_rotation: Rotation,
    pub coll_angle: f64,
    pub coll_offset_inplane: f64,
    pub coll_offset_crossplane: f64,
    pub scan_device: String,
    pub scan_device_setup: String,
    pub electrometer: String,
    pub range_field: String,
    pub range_reference: String,
    pub detector: String,
    pub detector_subcode: String,
    pub detector_radius: f64,
    pub detector_name: String,
    pub detector_sn: String,
    pub detector_calibration: f64,
    pub detector_is_calibrated: bool,
    pub detector_reference: String,
    pub detector_reference_subcode: String,
    pub detector_reference_radius: f64,
    pub detector_reference_name: String,
    pub detector_reference_sn: String,
    pub detector_reference_is_calibrated: bool,
    pub detector_reference_calibration: f64,
    pub detector_hv: f64,
    pub detector_reference_hv: f64,
    pub detector_orientation: Orientation,
    pub filter: String,
    pub scan_speed_profile: f64,
    pub scan_prof_speed_dep: String,
    pub scan_speed_pdd: f64,
    pub scan_pdd_speed_dep: String,
    pub detector_type: String,
    pub ref_field_depth: f64,
    pub ref_field_defined: String,
    pub ref_field_inplane: f64,
    pub ref_field_crossplane: f64,
    pub ref_scan_positions: Vec<f64>,
    pub ref_overscan_factor: f64,
    pub scan_curvetype: CurveType,
    pub scan_depth: f64,
    pub scan_offaxis_inplane: f64,
    pub scan_offaxis_crossplane: f64,
    pub scan_angle: f64,
    pub scan_diagonal: String,
    pub scan_direction: String,
    pub meas_medium: String,
    pub meas_preset: String,
    pub meas_time: f64,
    pub meas_unit: String,
    pub pressure: f64,
    pub temperature: f64,
    pub norm_temperature: f64,
    pub correction_factor: f64,
    pub expected_max_dose_rate: f64,
    pub epom_depth_shift: f64,
    pub epom_mode: String,
    pub data: Vec<Vec<f64>>,
}
