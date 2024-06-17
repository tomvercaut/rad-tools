use crate::data::{DetectorType, TaskType};
use serde::{Deserialize, Serialize};

/// Represents administrative information for a task.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Adminsitrative")]
pub struct Administrative {
    #[serde(rename = "TaskName")]
    pub task_name: TaskType,
    #[serde(rename = "Module")]
    pub module: String,
    #[serde(rename = "MeasDate")]
    pub measurement_date: String,
}

/// Represents a rotation unit for a task.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename = "RotationUnit")]
pub struct RotationUnit {
    /// Direction of rotation.
    #[serde(rename = "RURotationDirection")]
    pub rotation_direction: String,
    /// Mounting of the inclinometer (normal or inverse).
    #[serde(rename = "InclinometerMounting")]
    pub inclinometer_mounting: String,
}

/// Represents the settings for an accelerator.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename = "AcceleratorSettings")]
pub struct AcceleratorSettings {
    /// The angle of the gantry when the gantry in upright position.
    #[serde(rename = "GantryUprightPosition")]
    pub gantry_upright_position: String,
    /// The direction of rotation for the gantry.
    /// Possible values are "clockwise" and "counterclockwise".
    #[serde(rename = "GantryRotationDirection")]
    pub gantry_rotation_direction: String,
}

/// Represents an Inclinometer used during measurements.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inclinometer")]
pub struct Inclinometer {
    /// Serial number
    #[serde(rename = "InclinometerSN")]
    pub inclinometer_sn: String,
    /// Date and time of the first measurement
    #[serde(rename = "MeasDateFirstAngle")]
    pub measurement_date_first_angle: String,
    /// Angles measured
    #[serde(
        rename = "AngleValues",
        serialize_with = "crate::serdeh::SerdeBase64F32WithInternalF64::serialize",
        deserialize_with = "crate::serdeh::SerdeBase64F32WithInternalF64::deserialize"
    )]
    pub angle_values: Vec<f64>,
    /// Times at which the angles were measured.
    #[serde(
        rename = "AngleTimes",
        serialize_with = "crate::serdeh::SerdeBase64U64::serialize",
        deserialize_with = "crate::serdeh::SerdeBase64U64::deserialize"
    )]
    pub angle_times: Vec<u64>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename = "MeasuringDevice")]
pub struct MeasuringDevice {
    /// Detector type
    #[serde(rename = "Detector")]
    pub detector: DetectorType,
    /// Calibration file for the detector
    #[serde(rename = "DetectorCalibrationFileName")]
    pub detector_calibration_file_name: String,
    /// Electrometer to which the detector is connected
    #[serde(rename = "Electrometer")]
    pub electrometer: String,
    /// Serial number of the electrometer
    #[serde(rename = "ElectrometerSN")]
    pub electrometer_sn: String,
    /// Serial number of the detector
    #[serde(rename = "DetectorSN")]
    pub detector_sn: String,
    /// Device
    #[serde(rename = "ScanDevice")]
    pub scan_device: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "MeasurementParameters")]
pub struct MeasurementParameters {
    /// Depth at which the measurement is done
    #[serde(rename = "ScanDepth")]
    pub scan_depth: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Correction")]
pub struct Correction {
    /// Air pressure
    #[serde(rename = "CorrAirDensityPressure")]
    pub air_density_pressure: f64,
    /// Temperature of the air
    #[serde(rename = "CorrAirDensityTemperature")]
    pub air_density_temperature: f64,
    /// Reference air pressure
    #[serde(rename = "CorrAirDensityReferencePressure")]
    pub air_density_reference_pressure: f64,
    /// Reference temperature of the air
    #[serde(rename = "CorrAirDensityReferenceTemperature")]
    pub air_density_reference_temperature: f64,
    /// Beam energy
    #[serde(rename = "CorrEnergy")]
    pub energy: f64,
    /// Correction factor
    #[serde(rename = "CorrKUser")]
    pub k_user: f64,
    /// Correction flags
    #[serde(rename = "CorrFlags")]
    pub flags: String,
    #[serde(rename = "SystemSyncAdd")]
    pub system_sync_add: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DetectorArray")]
pub struct DetectorArray {
    /// Mode in which dose / charge is accumulated
    #[serde(rename = "DeviceStoreMode")]
    pub device_store_mode: String,
    /// Number of detectors in the array
    #[serde(
        rename = "DetectorNumbers",
        serialize_with = "crate::serdeh::SerdeBase64U16::serialize",
        deserialize_with = "crate::serdeh::SerdeBase64U16::deserialize"
    )]
    pub detector_numbers: Vec<u16>,
    /// Coordinate of the detector on the left
    #[serde(rename = "MatrixLeftCoordinate")]
    pub matrix_left_coordinate: f64,
    /// Coordinate of the detector on the direction of the gun (C-arm linac)
    #[serde(rename = "MatrixGunCoordinate")]
    pub matrix_gun_coordinate: f64,
    /// Spacing between the detectors (left right)
    #[serde(rename = "MatrixResolutionLR")]
    pub matrix_resolution_lr: f64,
    /// Spacing between the detectors (gun target)
    #[serde(rename = "MatrixResolutionGT")]
    pub matrix_resolution_gt: f64,
    /// Number of measurments in the left right direction
    #[serde(rename = "MatrixNumberOfMeasLR")]
    pub matrix_number_of_meas_lr: u32,
    /// Number of measurments in the gun target direction
    #[serde(rename = "MatrixNumberOfMeasGT")]
    pub matrix_number_of_meas_gt: u32,
    /// Size of the ionisation chamber in left right direction
    #[serde(rename = "ChamberDimensionLR")]
    pub chamber_dimension_lr: f64,
    /// Size of the ionisation chamber in gun target direction
    #[serde(rename = "ChamberDimensionGT")]
    pub chamber_dimension_gt: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "MeasurementPreset")]
pub struct MeasurementPreset {
    /// Measurement unit (e.g. PTW_MEASUNIT_GY)
    #[serde(rename = "MeasUnit")]
    pub meas_unit: String,
    /// Measurement preset (e.g. PTW_MEASUNIT_TIME)
    #[serde(rename = "MeasPreset")]
    pub meas_preset: String,
    /// Measurement time
    #[serde(rename = "MeasTime")]
    pub meas_time: f64,
    /// Interval time
    #[serde(rename = "IntervalTime")]
    pub interval_time: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Measurement")]
pub struct Measurement {
    /// Cumulative time at which the measurement is made.
    #[serde(rename = "Time")]
    pub time: f64,
    /// Gantry angle
    #[serde(rename = "Angle")]
    pub angle: f64,
    /// Data for all the detectors
    #[serde(
        rename = "Data",
        serialize_with = "crate::serdeh::SerdeBase64F32WithInternalF64::serialize",
        deserialize_with = "crate::serdeh::SerdeBase64F32WithInternalF64::deserialize"
    )]
    pub data: Vec<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MeasurementData {
    #[serde(rename = "Measurement")]
    pub measurements: Vec<Measurement>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Content {
    /// Administrative data
    #[serde(rename = "Adminsitrative")]
    pub administrative: Administrative,
    /// Rotation unit
    #[serde(rename = "RotationUnit")]
    pub rotation_unit: RotationUnit,
    /// Accelerator setup
    #[serde(rename = "AcceleratorSettings")]
    pub accelerator_settings: AcceleratorSettings,
    /// Inclinometer settings
    #[serde(rename = "Inclinometer")]
    pub inclinometer: Inclinometer,
    /// Measurement device parameters
    #[serde(rename = "MeasuringDevice")]
    pub measuring_device: MeasuringDevice,
    /// Parameters linked with the measurement
    #[serde(rename = "MeasurementParameters")]
    pub measurement_parameters: MeasurementParameters,
    /// Correction factor related parameters
    #[serde(rename = "Correction")]
    pub correction: Correction,
    /// Detector array parameters
    #[serde(rename = "DetectorArray")]
    pub detector_array: DetectorArray,
    /// Measurement preset
    #[serde(rename = "MeasurementPreset")]
    pub measurement_preset: MeasurementPreset,
    /// Measured data
    #[serde(rename = "MeasData")]
    pub measurement_data: MeasurementData,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "PTW")]
pub struct Xcc {
    /// File format version
    #[serde(rename = "Version")]
    pub version: String,
    /// Timestamp at which the file was created / last modified
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    /// Data content
    #[serde(rename = "Content")]
    pub content: Content,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_xml_rs::from_str;

    #[test]
    fn serde_xml_rotation_unit() {
        let s = r#"
    <RotationUnit>
      <RURotationDirection>ptwRURotationDirection_SameAsLinac</RURotationDirection>
      <InclinometerMounting>ptwInclinometerMounting_Normal</InclinometerMounting>
    </RotationUnit>
        "#;
        let expected = RotationUnit {
            rotation_direction: "ptwRURotationDirection_SameAsLinac".to_string(),
            inclinometer_mounting: "ptwInclinometerMounting_Normal".to_string(),
        };
        let ru = from_str(s).unwrap();
        assert_eq!(expected, ru);
    }

    #[test]
    fn serde_xml_administrative() {
        let s = r#"
    <Adminsitrative>
      <TaskName><![CDATA[2D_ARRAY_MEASUREMENT]]></TaskName>
      <Module><![CDATA[VeriSoft 8.0.1.0]]></Module>
      <MeasDate><![CDATA[2023-12-05 17:57:22.219]]></MeasDate>
    </Adminsitrative>
        "#;
        let expected = Administrative {
            task_name: TaskType::Measurement2dArray,
            module: "VeriSoft 8.0.1.0".to_string(),
            measurement_date: "2023-12-05 17:57:22.219".to_string(),
        };
        let admin = from_str(s).unwrap();
        assert_eq!(expected, admin);
    }
}
