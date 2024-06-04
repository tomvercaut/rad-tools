use std::ops::Deref;
use std::path::Path;
use std::str::FromStr;

use crate::data::{DetectorType, TaskType};
use base64::Engine;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::data::xcc::{Measurement, Xcc};
use crate::{decode, DosimetryToolsError};

#[derive(Clone, Debug, Eq, PartialEq)]
enum State {
    None,
    Root,
    Version,
    LastModified,
    Content,
    Administrative,
    TaskName,
    Module,
    MeasDate,
    RotationUnit,
    RuRotationDirection,
    InclinometerMounting,
    AcceleratorSettings,
    GantryUprightPosition,
    GantryRotationDirection,
    Inclinometer,
    InclinometerSn,
    MeasDateFirstAngle,
    AngleValues,
    AngleTimes,
    MeasuringDevice,
    Detector,
    DetectorCalibrationFileName,
    Electrometer,
    ElectrometerSN,
    DetectorSN,
    ScanDevice,
    MeasurementParameters,
    ScanDepth,
    Correction,
    CorrAirDensityPressure,
    CorrAirDensityTemperature,
    CorrAirDensityReferencePressure,
    CorrAirDensityReferenceTemperature,
    CorrEnergy,
    CorrKUser,
    CorrFlags,
    SystemSyncAdd,
    DetectorArray,
    DeviceStoreMode,
    DetectorNumbers,
    MatrixLeftCoordinate,
    MatrixGunCoordinate,
    MatrixResolutionLR,
    MatrixResolutionGT,
    MatrixNumberOfMeasLR,
    MatrixNumberOfMeasGT,
    ChamberDimensionLR,
    ChamberDimensionGT,
    MeasurementPreset,
    MeasUnit,
    MeasPreset,
    MeasTime,
    IntervalTime,
    MeasData,
    Measurement,
    Time,
    Angle,
    Data,
}

impl Default for State {
    fn default() -> Self {
        Self::None
    }
}

/// Reads an XCC XML file and returns a `Xcc` struct.
///
/// # Arguments
///
/// * `path` - A path to the XCC file.
///
/// # Returns
///
/// A `Result` containing the `Xcc` struct or an error of type `DosimetryToolsError` if there was an issue
/// reading the file or parsing the XML.
///
/// # Example
///
/// ```no_run
/// use peeteewee::{DosimetryToolsError, data::xcc::Xcc};
///
/// let result = peeteewee::io::xcc::read("data.xcc");
///
/// match result {
///     Ok(xcc) => {
///         // Do something with the Xcc struct
///     }
///     Err(err) => {
///         // Handle the error
///     }
/// }
/// ```
pub fn read<P: AsRef<Path>>(path: P) -> Result<Xcc, DosimetryToolsError> {
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);
    let mut xcc = Xcc::default();
    let mut buf = vec![];
    let mut states = vec![];
    let mut t_measurement = Measurement::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(element) => {
                let qname = element.name();
                let name = qname.as_ref();
                if b"PTW" == name {
                    states.push(State::Root);
                } else if b"Version" == name {
                    states.push(State::Version);
                } else if b"LastModified" == name {
                    states.push(State::LastModified);
                } else if b"Content" == name {
                    states.push(State::Content);
                } else if b"Adminsitrative" == name {
                    states.push(State::Administrative);
                } else if b"TaskName" == name {
                    states.push(State::TaskName);
                } else if b"Module" == name {
                    states.push(State::Module);
                } else if b"MeasDate" == name {
                    states.push(State::MeasDate);
                } else if b"RotationUnit" == name {
                    states.push(State::RotationUnit);
                } else if b"RURotationDirection" == name {
                    states.push(State::RuRotationDirection);
                } else if b"InclinometerMounting" == name {
                    states.push(State::InclinometerMounting);
                } else if b"AcceleratorSettings" == name {
                    states.push(State::AcceleratorSettings);
                } else if b"GantryUprightPosition" == name {
                    states.push(State::GantryUprightPosition);
                } else if b"GantryRotationDirection" == name {
                    states.push(State::GantryRotationDirection);
                } else if b"Inclinometer" == name {
                    states.push(State::Inclinometer);
                } else if b"InclinometerSN" == name {
                    states.push(State::InclinometerSn);
                } else if b"MeasDateFirstAngle" == name {
                    states.push(State::MeasDateFirstAngle);
                } else if b"AngleValues" == name {
                    states.push(State::AngleValues);
                } else if b"AngleTimes" == name {
                    states.push(State::AngleTimes);
                } else if b"MeasuringDevice" == name {
                    states.push(State::MeasuringDevice);
                } else if b"Detector" == name {
                    states.push(State::Detector);
                } else if b"DetectorCalibrationFileName" == name {
                    states.push(State::DetectorCalibrationFileName);
                } else if b"Electrometer" == name {
                    states.push(State::Electrometer);
                } else if b"ElectrometerSN" == name {
                    states.push(State::ElectrometerSN);
                } else if b"DetectorSN" == name {
                    states.push(State::DetectorSN);
                } else if b"ScanDevice" == name {
                    states.push(State::ScanDevice);
                } else if b"MeasurementParameters" == name {
                    states.push(State::MeasurementParameters);
                } else if b"ScanDepth" == name {
                    states.push(State::ScanDepth);
                } else if b"Correction" == name {
                    states.push(State::Correction);
                } else if b"CorrAirDensityPressure" == name {
                    states.push(State::CorrAirDensityPressure);
                } else if b"CorrAirDensityTemperature" == name {
                    states.push(State::CorrAirDensityTemperature);
                } else if b"CorrAirDensityReferencePressure" == name {
                    states.push(State::CorrAirDensityReferencePressure);
                } else if b"CorrAirDensityReferenceTemperature" == name {
                    states.push(State::CorrAirDensityReferenceTemperature);
                } else if b"CorrEnergy" == name {
                    states.push(State::CorrEnergy);
                } else if b"CorrKUser" == name {
                    states.push(State::CorrKUser);
                } else if b"CorrFlags" == name {
                    states.push(State::CorrFlags);
                } else if b"SystemSyncAdd" == name {
                    states.push(State::SystemSyncAdd);
                } else if b"DetectorArray" == name {
                    states.push(State::DetectorArray);
                } else if b"DeviceStoreMode" == name {
                    states.push(State::DeviceStoreMode);
                } else if b"DetectorNumbers" == name {
                    states.push(State::DetectorNumbers);
                } else if b"MatrixLeftCoordinate" == name {
                    states.push(State::MatrixLeftCoordinate);
                } else if b"MatrixGunCoordinate" == name {
                    states.push(State::MatrixGunCoordinate);
                } else if b"MatrixResolutionLR" == name {
                    states.push(State::MatrixResolutionLR);
                } else if b"MatrixResolutionGT" == name {
                    states.push(State::MatrixResolutionGT);
                } else if b"MatrixNumberOfMeasLR" == name {
                    states.push(State::MatrixNumberOfMeasLR);
                } else if b"MatrixNumberOfMeasGT" == name {
                    states.push(State::MatrixNumberOfMeasGT);
                } else if b"ChamberDimensionLR" == name {
                    states.push(State::ChamberDimensionLR);
                } else if b"ChamberDimensionGT" == name {
                    states.push(State::ChamberDimensionGT);
                } else if b"MeasurementPreset" == name {
                    states.push(State::MeasurementPreset);
                } else if b"MeasUnit" == name {
                    states.push(State::MeasUnit);
                } else if b"MeasPreset" == name {
                    states.push(State::MeasPreset);
                } else if b"MeasTime" == name {
                    states.push(State::MeasTime);
                } else if b"IntervalTime" == name {
                    states.push(State::IntervalTime);
                } else if b"MeasData" == name {
                    states.push(State::MeasData);
                } else if b"Measurement" == name {
                    states.push(State::Measurement);
                    t_measurement = Measurement::default();
                } else if b"Time" == name {
                    states.push(State::Time);
                } else if b"Angle" == name {
                    states.push(State::Angle);
                } else if b"Data" == name {
                    states.push(State::Data);
                } else {
                    return Err(DosimetryToolsError::UndefinedXMLElement(
                        std::str::from_utf8(name)?.to_string(),
                    ));
                }
            }
            Event::End(element) => {
                let qname = element.name();
                let name = qname.as_ref();
                if b"Measurement" == name {
                    xcc.content.measurement_data.push(t_measurement);
                    t_measurement = Measurement::default();
                }
                states.pop();
            }
            Event::Empty(_) => {}
            Event::Text(text) => {
                let last_state = states.last().unwrap();
                let text = std::str::from_utf8(text.deref())?.to_string();
                match last_state {
                    State::Version => {
                        xcc.version = text;
                    }
                    State::LastModified => {
                        xcc.last_modified = text;
                    }
                    State::RuRotationDirection => {
                        xcc.content.rotation_unit.rotation_direction = text;
                    }
                    State::InclinometerMounting => {
                        xcc.content.rotation_unit.inclinometer_mounting = text;
                    }
                    State::AcceleratorSettings => {}
                    State::GantryUprightPosition => {
                        xcc.content.accelerator_settings.gantry_upright_position = text;
                    }
                    State::GantryRotationDirection => {
                        xcc.content.accelerator_settings.gantry_rotation_direction = text;
                    }

                    State::AngleValues => {
                        xcc.content.inclinometer.angle_values = decode::base64_f32s_as_f64s(text)?;
                    }
                    State::AngleTimes => {
                        xcc.content.inclinometer.angle_times = decode::base64_to_u64s(text)?;
                    }
                    State::Detector => {
                        xcc.content.measuring_device.detector =
                            DetectorType::from_str(text.as_str())?;
                    }
                    State::DetectorCalibrationFileName => {}
                    State::Electrometer => {
                        xcc.content.measuring_device.electrometer = text;
                    }
                    State::ScanDevice => {
                        xcc.content.measuring_device.scan_device = text;
                    }
                    State::ScanDepth => {
                        xcc.content.measurement_parameters.scan_depth = decode::to_f64(text)?;
                    }
                    State::CorrAirDensityPressure => {
                        xcc.content.correction.air_density_pressure = decode::to_f64(text)?;
                    }
                    State::CorrAirDensityTemperature => {
                        xcc.content.correction.air_density_temperature = decode::to_f64(text)?;
                    }
                    State::CorrAirDensityReferencePressure => {
                        xcc.content.correction.air_density_reference_pressure =
                            decode::to_f64(text)?;
                    }
                    State::CorrAirDensityReferenceTemperature => {
                        xcc.content.correction.air_density_reference_temperature =
                            decode::to_f64(text)?;
                    }
                    State::CorrEnergy => {
                        xcc.content.correction.energy = decode::to_f64(text)?;
                    }
                    State::CorrKUser => {
                        xcc.content.correction.k_user = decode::to_f64(text)?;
                    }
                    State::CorrFlags => {
                        xcc.content.correction.flags = text;
                    }
                    State::DeviceStoreMode => {
                        xcc.content.detector_array.device_store_mode = text;
                    }
                    State::DetectorNumbers => {
                        xcc.content.detector_array.detector_numbers = decode::base64_u16s(text)?;
                    }
                    State::MatrixLeftCoordinate => {
                        xcc.content.detector_array.matrix_left_coordinate = decode::to_f64(text)?;
                    }
                    State::MatrixGunCoordinate => {
                        xcc.content.detector_array.matrix_gun_coordinate = decode::to_f64(text)?;
                    }
                    State::MatrixResolutionLR => {
                        xcc.content.detector_array.matrix_resolution_lr = decode::to_f64(text)?;
                    }
                    State::MatrixResolutionGT => {
                        xcc.content.detector_array.matrix_resolution_gt = decode::to_f64(text)?;
                    }
                    State::MatrixNumberOfMeasLR => {
                        xcc.content.detector_array.matrix_number_of_meas_lr = decode::to_u32(text)?;
                    }
                    State::MatrixNumberOfMeasGT => {
                        xcc.content.detector_array.matrix_number_of_meas_gt = decode::to_u32(text)?;
                    }
                    State::ChamberDimensionLR => {
                        xcc.content.detector_array.chamber_dimension_lr = decode::to_f64(text)?;
                    }
                    State::ChamberDimensionGT => {
                        xcc.content.detector_array.chamber_dimension_gt = decode::to_f64(text)?;
                    }
                    State::MeasUnit => {
                        xcc.content.measurement_preset.meas_unit = text;
                    }
                    State::MeasPreset => {
                        xcc.content.measurement_preset.meas_preset = text;
                    }
                    State::MeasTime => {
                        xcc.content.measurement_preset.meas_time = decode::to_f64(text)?;
                    }
                    State::IntervalTime => {
                        xcc.content.measurement_preset.interval_time = decode::to_f64(text)?;
                    }
                    State::Time => {
                        t_measurement.time = decode::to_f64(text)?;
                    }
                    State::Angle => {
                        t_measurement.angle = decode::to_f64(text)?;
                    }
                    State::Data => {
                        t_measurement.data = decode::base64_f32s_as_f64s(text)?;
                    }
                    _ => {}
                }
            }
            Event::CData(text) => {
                let last_state = states.last().unwrap();
                let text = std::str::from_utf8(text.deref())?.to_string();
                match last_state {
                    State::TaskName => {
                        xcc.content.administrative.task_name = TaskType::from_str(text.as_str())?;
                    }
                    State::Module => {
                        xcc.content.administrative.module = text;
                    }
                    State::MeasDate => {
                        xcc.content.administrative.measurement_date = text;
                    }
                    State::InclinometerSn => {
                        xcc.content.inclinometer.inclinometer_sn = text;
                    }
                    State::MeasDateFirstAngle => {
                        xcc.content.inclinometer.measurement_date_first_angle = text;
                    }
                    State::DetectorCalibrationFileName => {
                        xcc.content.measuring_device.detector_calibration_file_name = text;
                    }
                    State::ElectrometerSN => {
                        xcc.content.measuring_device.electrometer_sn = text;
                    }
                    State::DetectorSN => {
                        xcc.content.measuring_device.detector_sn = text;
                    }
                    State::SystemSyncAdd => {
                        xcc.content.correction.system_sync_add = text;
                    }
                    _ => {}
                }
            }
            Event::Comment(_) => {}
            Event::Decl(_) => {}
            Event::PI(_) => {}
            Event::DocType(_) => {}
            Event::Eof => {
                break;
            }
        }
    }

    Ok(xcc)
}
