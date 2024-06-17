use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

use crate::data::mcc::{CurveType, Mcc, Orientation, Rotation, Scan};
use crate::PeeTeeWeeError;

struct State<'a> {
    lines: &'a Vec<String>,
    i: usize,
    n: usize,
}

impl<'a> State<'a> {
    pub fn new(lines: &'a Vec<String>) -> Self {
        Self {
            lines,
            i: 0,
            n: lines.len(),
        }
    }

    pub fn next(&mut self) -> Option<&str> {
        if self.i >= self.n {
            None
        } else {
            let t = self.lines.get(self.i).as_ref().unwrap().as_str();
            self.i += 1;
            Some(t)
        }
    }

    pub fn has_next(&self) -> bool {
        self.i < self.n
    }
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<Mcc, PeeTeeWeeError> {
    let p = path.as_ref();
    let file = File::open(p)?;
    let rdr = BufReader::new(file);
    let mut lines = vec![];
    for line in std::io::read_to_string(rdr)?.lines() {
        lines.push(line.trim().to_string());
    }
    let mut state = State::new(&lines);
    let mut has_next = state.has_next();
    let mut mcc = Mcc::default();
    while has_next {
        let s = state.next().unwrap();
        let split: Vec<&str> = s.split('=').collect();
        let key = *split.first().unwrap();
        if key == "FORMAT" {
            mcc.format = read_str_value(&split)?;
        } else if key == "FILE_CREATION_DATE" {
            mcc.file_creation_date = read_str_value(&split)?;
        } else if key == "LAST_MODIFIED" {
            mcc.last_modified = read_str_value(&split)?;
        } else if key.starts_with("BEGIN_SCAN") && key != "BEGIN_SCAN_DATA" {
            let (tstate, scan) = read_scan(state)?;
            state = tstate;
            mcc.scans.push(scan)
        } else if key.starts_with("END_SCAN_DATA") {
            break;
        }
        has_next = state.has_next();
    }
    Ok(mcc)
}

fn read_scan(state: State) -> Result<(State, Scan), PeeTeeWeeError> {
    let mut state = state;
    let mut scan = Scan::default();
    let mut has_line = state.has_next();

    while has_line {
        let line = state.next().unwrap();
        let line = line.trim();
        if line.starts_with("END_SCAN") {
            break;
        }
        let v: Vec<&str> = line.split('=').collect();
        let key = *v.first().unwrap();
        if key == "TASK_NAME" {
            scan.task_name = read_str_value(&v)?;
        } else if key == "PROGRAM" {
            scan.program = read_str_value(&v)?;
        } else if key == "MEAS_DATE" {
            scan.meas_date = read_str_value(&v)?;
        } else if key == "LINAC" {
            scan.linac = read_str_value(&v)?;
        } else if key == "MODALITY" {
            scan.modality = read_str_value(&v)?;
        } else if key == "ISOCENTER" {
            scan.isocenter = read_f64_value(&v)?;
        } else if key == "INPLANE_AXIS" {
            scan.inplane_axis = read_str_value(&v)?;
        } else if key == "CROSSPLANE_AXIS" {
            scan.crossplane_axis = read_str_value(&v)?;
        } else if key == "DEPTH_AXIS" {
            scan.depth_axis = read_str_value(&v)?;
        } else if key == "INPLANE_AXIS_DIR" {
            scan.inplane_axis_dir = read_str_value(&v)?;
        } else if key == "CROSSPLANE_AXIS_DIR" {
            scan.crossplane_axis_dir = read_str_value(&v)?;
        } else if key == "DEPTH_AXIS_DIR" {
            scan.depth_axis_dir = read_str_value(&v)?;
        } else if key == "ENERGY" {
            scan.energy = read_f64_value(&v)?;
        } else if key == "NOMINAL_DMAX" {
            scan.nominal_dmax = read_f64_value(&v)?;
        } else if key == "SSD" {
            scan.ssd = read_f64_value(&v)?;
        } else if key == "BLOCK" {
            scan.block = read_int_as_bool_value(&v)?;
        } else if key == "WEDGE_ANGLE" {
            scan.wedge_angle = read_f64_value(&v)?;
        } else if key == "FIELD_INPLANE" {
            scan.field_inplane = read_f64_value(&v)?;
        } else if key == "FIELD_CROSSPLANE" {
            scan.field_crossplane = read_f64_value(&v)?;
        } else if key == "FIELD_TYPE" {
            scan.field_type = read_str_value(&v)?;
        } else if key == "GANTRY" {
            scan.gantry = read_f64_value(&v)?;
        } else if key == "GANTRY_UPRIGHT_POSITION" {
            scan.gantry_upright_position = read_int_as_bool_value(&v)?;
        } else if key == "GANTRY_ROTATION" {
            scan.gantry_rotation = Rotation::from_str(read_str_value(&v)?.as_str())?;
        } else if key == "COLL_ANGLE" {
            scan.coll_angle = read_f64_value(&v)?;
        } else if key == "COLL_OFFSET_INPLANE" {
            scan.coll_offset_inplane = read_f64_value(&v)?;
        } else if key == "COLL_OFFSET_CROSSPLANE" {
            scan.coll_offset_crossplane = read_f64_value(&v)?;
        } else if key == "SCAN_DEVICE" {
            scan.scan_device = read_str_value(&v)?;
        } else if key == "SCAN_DEVICE_SETUP" {
            scan.scan_device_setup = read_str_value(&v)?;
        } else if key == "ELECTROMETER" {
            scan.electrometer = read_str_value(&v)?;
        } else if key == "RANGE_FIELD" {
            scan.range_field = read_str_value(&v)?;
        } else if key == "RANGE_REFERENCE" {
            scan.range_reference = read_str_value(&v)?;
        } else if key == "DETECTOR" {
            scan.detector = read_str_value(&v)?;
        } else if key == "DETECTOR_SUBCODE" {
            scan.detector_subcode = read_str_value(&v)?;
        } else if key == "DETECTOR_RADIUS" {
            scan.detector_radius = read_f64_value(&v)?;
        } else if key == "DETECTOR_NAME" {
            scan.detector_name = read_str_value(&v)?;
        } else if key == "DETECTOR_SN" {
            scan.detector_sn = read_str_value(&v)?;
        } else if key == "DETECTOR_CALIBRATION" {
            scan.detector_calibration = read_f64_value(&v)?;
        } else if key == "DETECTOR_IS_CALIBRATED" {
            scan.detector_is_calibrated = read_int_as_bool_value(&v)?;
        } else if key == "DETECTOR_REFERENCE" {
            scan.detector_reference = read_str_value(&v)?;
        } else if key == "DETECTOR_REFERENCE_SUBCODE" {
            scan.detector_reference_subcode = read_str_value(&v)?;
        } else if key == "DETECTOR_REFERENCE_RADIUS" {
            scan.detector_reference_radius = read_f64_value(&v)?;
        } else if key == "DETECTOR_REFERENCE_NAME" {
            scan.detector_reference_name = read_str_value(&v)?;
        } else if key == "DETECTOR_REFERENCE_SN" {
            scan.detector_reference_sn = read_str_value(&v)?;
        } else if key == "DETECTOR_REFERENCE_IS_CALIBRATED" {
            scan.detector_reference_is_calibrated = read_int_as_bool_value(&v)?;
        } else if key == "DETECTOR_REFERENCE_CALIBRATION" {
            scan.detector_reference_calibration = read_f64_value(&v)?;
        } else if key == "DETECTOR_HV" {
            scan.detector_hv = read_f64_value(&v)?;
        } else if key == "DETECTOR_REFERENCE_HV" {
            scan.detector_reference_hv = read_f64_value(&v)?;
        } else if key == "DETECTOR_ORIENTATION" {
            scan.detector_orientation = Orientation::from_str(read_str_value(&v)?.as_str())?;
        } else if key == "FILTER" {
            scan.filter = read_str_value(&v)?;
        } else if key == "SCAN_SPEED_PROFILE" {
            scan.scan_speed_profile = read_f64_value(&v)?;
        } else if key == "SCAN_PROF_SPEED_DEP" {
            scan.scan_prof_speed_dep = read_str_value(&v)?;
        } else if key == "SCAN_SPEED_PDD" {
            scan.scan_speed_pdd = read_f64_value(&v)?;
        } else if key == "SCAN_PDD_SPEED_DEP" {
            scan.scan_pdd_speed_dep = read_str_value(&v)?;
        } else if key == "DETECTOR_TYPE" {
            scan.detector_type = read_str_value(&v)?;
        } else if key == "REF_FIELD_DEPTH" {
            scan.ref_field_depth = read_f64_value(&v)?;
        } else if key == "REF_FIELD_DEFINED" {
            scan.ref_field_defined = read_str_value(&v)?;
        } else if key == "REF_FIELD_INPLANE" {
            scan.ref_field_inplane = read_f64_value(&v)?;
        } else if key == "REF_FIELD_CROSSPLANE" {
            scan.ref_field_crossplane = read_f64_value(&v)?;
        } else if key == "REF_SCAN_POSITIONS" {
            scan.ref_scan_positions = read_f64_values(&v)?;
        } else if key == "REF_OVERSCAN_FACTOR" {
            scan.ref_overscan_factor = read_f64_value(&v)?;
        } else if key == "SCAN_CURVETYPE" {
            scan.scan_curvetype = CurveType::from_str(read_str_value(&v)?.as_str())?;
        } else if key == "SCAN_DEPTH" {
            scan.scan_depth = read_f64_value(&v)?;
        } else if key == "SCAN_OFFAXIS_INPLANE" {
            scan.scan_offaxis_inplane = read_f64_value(&v)?;
        } else if key == "SCAN_OFFAXIS_CROSSPLANE" {
            scan.scan_offaxis_crossplane = read_f64_value(&v)?;
        } else if key == "SCAN_ANGLE" {
            scan.scan_angle = read_f64_value(&v)?;
        } else if key == "SCAN_DIAGONAL" {
            scan.scan_diagonal = read_str_value(&v)?;
        } else if key == "SCAN_DIRECTION" {
            scan.scan_direction = read_str_value(&v)?;
        } else if key == "MEAS_MEDIUM" {
            scan.meas_medium = read_str_value(&v)?;
        } else if key == "MEAS_PRESET" {
            scan.meas_preset = read_str_value(&v)?;
        } else if key == "MEAS_TIME" {
            scan.meas_time = read_f64_value(&v)?;
        } else if key == "MEAS_UNIT" {
            scan.meas_unit = read_str_value(&v)?;
        } else if key == "PRESSURE" {
            scan.pressure = read_f64_value(&v)?;
        } else if key == "TEMPERATURE" {
            scan.temperature = read_f64_value(&v)?;
        } else if key == "NORM_TEMPERATURE" {
            scan.norm_temperature = read_f64_value(&v)?;
        } else if key == "CORRECTION_FACTOR" {
            scan.correction_factor = read_f64_value(&v)?;
        } else if key == "EXPECTED_MAX_DOSE_RATE" {
            scan.expected_max_dose_rate = read_f64_value(&v)?;
        } else if key == "EPOM_DEPTH_SHIFT" {
            scan.epom_depth_shift = read_f64_value(&v)?;
        } else if key == "EPOM_MODE" {
            scan.epom_mode = read_str_value(&v)?;
        } else if key.starts_with("BEGIN_DATA") {
            let (tstate, scan_data) = read_scan_data(state)?;
            state = tstate;
            scan.data = scan_data
        }

        has_line = state.has_next();
    }

    Ok((state, scan))
}

fn read_scan_data(state: State) -> Result<(State, Vec<Vec<f64>>), PeeTeeWeeError> {
    let mut state = state;
    let mut has_line = state.has_next();
    let mut vvf: Vec<Vec<f64>> = vec![];

    while has_line {
        let line = state.next().unwrap();
        let line = line.trim_start();
        if line.starts_with("END_DATA") {
            break;
        }
        let split = line.split('\t');
        let mut vf = vec![];
        for s in split {
            if !s.is_empty() {
                vf.push(s.trim().parse::<f64>()?);
            }
        }
        vvf.push(vf);
        has_line = state.has_next();
    }
    Ok((state, vvf))
}

/// Reads the second string value from a slice of string references.
///
/// # Arguments
///
/// * `v` - A slice of string references.
///
/// # Returns
///
/// Returns a `Result` containing the string value if successful, or
/// a `DosimetryToolsError` if unable to retrieve the value.
///
/// # Errors
///
/// This function will return a `DosimetryToolsError::KeyValueString` error
/// if the slice `v` is empty or if the second element in the slice is not present.
pub(crate) fn read_str_value(v: &[&str]) -> Result<String, PeeTeeWeeError> {
    let o = v.get(1);
    let s = o.ok_or(PeeTeeWeeError::KeyValueString(
        v.first().unwrap().to_string(),
    ))?;
    Ok(s.to_string())
}

/// Reads a `f64` value from the second string of given slice of strings.
///
/// # Arguments
///
/// * `v` - A slice of string references, where the value is expected to be present.
///
/// # Returns
///
/// Returns a Result containing the parsed `f64` value if successful, or a `DosimetryToolsError` if an error occurred.
///
/// # Errors
///
/// This function will return a `DosimetryToolsError` under the following circumstances:
///
/// * If the input string cannot be parsed as a `f64` value.
/// * If there is an error while reading the string value.
fn read_f64_value(v: &[&str]) -> Result<f64, PeeTeeWeeError> {
    let s = read_str_value(v)?;
    let r = s.parse::<f64>();
    r.map_err(PeeTeeWeeError::from)
}

/// Reads the second string from a slice of string values and
/// returns a vector of parsed f64 values.
///
/// # Arguments
///
/// * `v` - A reference to a slice of string values.
///
/// # Returns
///
/// * `Result<Vec<f64>, DosimetryToolsError>` - A Result enum indicating success or failure.
///   * If successful, it contains a vector of parsed f64 values.
///   * If unsuccessful, it contains a DosimetryToolsError indicating the encountered error.
fn read_f64_values(v: &[&str]) -> Result<Vec<f64>, PeeTeeWeeError> {
    let s = read_str_value(v)?;
    let mut vf = vec![];
    for t in s.split(';') {
        if t.is_empty() {
            continue;
        }
        let t = t.parse::<f64>()?;
        vf.push(t);
    }
    Ok(vf)
}

fn read_int_as_bool_value(v: &[&str]) -> Result<bool, PeeTeeWeeError> {
    let s = read_str_value(v)?;
    if s == "1" {
        Ok(true)
    } else if s == "0" {
        Ok(false)
    } else {
        Err(PeeTeeWeeError::ParseBoolError(s))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn read_str_value() {
        let v = vec!["KEY", "value"];
        let r = super::read_str_value(&v);
        assert!(r.is_ok());
        assert_eq!("value".to_string(), r.unwrap());
    }

    #[test]
    fn read_str_value_err() {
        let v = vec!["KEY"];
        let r = super::read_str_value(&v);
        assert!(r.is_err());
    }

    #[test]
    fn read_f64_value() {
        let v = vec!["KEY", "0.5"];
        let r = super::read_f64_value(&v);
        assert!(r.is_ok());
        assert_eq!(0.5, r.unwrap());
    }

    #[test]
    fn read_f64_value_err() {
        let v = vec!["KEY"];
        let r = super::read_f64_value(&v);
        assert!(r.is_err());
    }

    #[test]
    fn read_int_as_bool_value() {
        let mut v = vec!["KEY", "0"];
        let mut r = super::read_int_as_bool_value(&v);
        assert!(r.is_ok());
        assert!(!r.unwrap());

        v = vec!["KEY", "1"];
        r = super::read_int_as_bool_value(&v);
        assert!(r.is_ok());
        assert!(r.unwrap());
    }

    #[test]
    fn read_int_as_bool_value_err() {
        let v = vec!["KEY", "2"];
        let r = super::read_int_as_bool_value(&v);
        assert!(r.is_err());
    }
}
