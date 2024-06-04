use crate::data::{DetectorType, TaskType};

/// Represents administrative information for a task.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Administrative {
    pub task_name: TaskType,
    pub module: String,
    pub measurement_date: String,
}

/// Represents a rotation unit for a task.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RotationUnit {
    pub rotation_direction: String,
    pub inclinometer_mounting: String,
}

/// Represents the settings for an accelerator.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AcceleratorSettings {
    /// The angle of the gantry when the gantry in upright position.
    pub gantry_upright_position: String,
    /// The direction of rotation for the gantry.
    /// Possible values are "clockwise" and "counterclockwise".
    pub gantry_rotation_direction: String,
}

/// Represents an Inclinometer used during measurements.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Inclinometer {
    /// Serial number
    pub inclinometer_sn: String,
    /// Date and time of the first measurement
    pub measurement_date_first_angle: String,
    /// Angles measured
    pub angle_values: Vec<f64>,
    /// Times at which the angles were measured.
    pub angle_times: Vec<u64>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MeasuringDevice {
    /// Detector type
    pub detector: DetectorType,
    /// Calibration file for the detector
    pub detector_calibration_file_name: String,
    /// Electrometer to which the detector is connected
    pub electrometer: String,
    /// Serial number of the electrometer
    pub electrometer_sn: String,
    /// Serial number of the detector
    pub detector_sn: String,
    /// Device
    pub scan_device: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MeasurementParameters {
    /// Depth at which the measurement is done
    pub scan_depth: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Correction {
    /// Air pressure
    pub air_density_pressure: f64,
    /// Temperature of the air
    pub air_density_temperature: f64,
    /// Reference air pressure
    pub air_density_reference_pressure: f64,
    /// Reference temperature of the air
    pub air_density_reference_temperature: f64,
    /// Beam energy
    pub energy: f64,
    /// Correction factor
    pub k_user: f64,
    /// Correction flags
    pub flags: String,
    pub system_sync_add: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DetectorArray {
    /// Mode in which dose / charge is accumulated
    pub device_store_mode: String,
    /// Number of detectors in the array
    pub detector_numbers: Vec<u16>,
    /// Coordinate of the detector on the left
    pub matrix_left_coordinate: f64,
    /// Coordinate of the detector on the direction of the gun (C-arm linac)
    pub matrix_gun_coordinate: f64,
    /// Spacing between the detectors (left right)
    pub matrix_resolution_lr: f64,
    /// Spacing between the detectors (gun target)
    pub matrix_resolution_gt: f64,
    /// Number of measurments in the left right direction
    pub matrix_number_of_meas_lr: u32,
    /// Number of measurments in the gun target direction
    pub matrix_number_of_meas_gt: u32,
    /// Size of the ionisation chamber in left right direction
    pub chamber_dimension_lr: f64,
    /// Size of the ionisation chamber in gun target direction
    pub chamber_dimension_gt: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MeasurementPreset {
    /// Measurement unit (e.g. PTW_MEASUNIT_GY)
    pub meas_unit: String,
    /// Measurement preset (e.g. PTW_MEASUNIT_TIME)
    pub meas_preset: String,
    /// Measurement time
    pub meas_time: f64,
    /// Interval time
    pub interval_time: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Measurement {
    /// Cumulative time at which the measurement is made.
    pub time: f64,
    /// Gantry angle
    pub angle: f64,
    /// Data for all the detectors
    pub data: Vec<f64>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Content {
    /// Administrative data
    pub administrative: Administrative,
    /// Rotation unit
    pub rotation_unit: RotationUnit,
    /// Accelerator setup
    pub accelerator_settings: AcceleratorSettings,
    /// Inclinometer settings
    pub inclinometer: Inclinometer,
    /// Measurement device parameters
    pub measuring_device: MeasuringDevice,
    /// Parameters linked with the measurement
    pub measurement_parameters: MeasurementParameters,
    /// Correction factor related parameters
    pub correction: Correction,
    /// Detector array parameters
    pub detector_array: DetectorArray,
    /// Measurement preset
    pub measurement_preset: MeasurementPreset,
    /// Measured data
    pub measurement_data: Vec<Measurement>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Xcc {
    /// File format version
    pub version: String,
    /// Timestamp at which the file was created / last modified
    pub last_modified: String,
    /// Data content
    pub content: Content,
}
