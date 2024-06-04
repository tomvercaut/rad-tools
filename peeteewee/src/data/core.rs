use std::str::FromStr;

use crate::DosimetryToolsError;

/// Represents the type of task.
///
/// The `TaskType` enum has two variants: None and Measurement2dArray.
///
/// # Examples
///
/// ```
/// use peeteewee::data::TaskType;
///
/// let none = TaskType::None;
/// let measurement_2d_array = TaskType::Measurement2dArray;
/// ```
///
/// `None` denotes that no task is selected.
///
/// `Measurement2dArray` represents a task involving a two-dimensional array of measurements.
///
/// # Notes
///
/// - The `Clone`, `Debug`, `Eq`, and `PartialEq` traits are implemented for `TaskType`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskType {
    None,
    Measurement2dArray,
}

impl Default for TaskType {
    fn default() -> Self {
        Self::None
    }
}

impl FromStr for TaskType {
    type Err = DosimetryToolsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "2D_ARRAY_MEASUREMENT" {
            Ok(Self::Measurement2dArray)
        } else {
            Err(DosimetryToolsError::ParseTaskTypeError(s.to_string()))
        }
    }
}

/// Represents the type of detector.
///
/// The `DetectorType` enum has two variants: None and Octavius1500.
///
/// # Examples
///
/// ```
/// use peeteewee::data::DetectorType;
///
/// let none = DetectorType::None;
/// let octavius = DetectorType::Octavius1500;
/// ```
///
/// `None` denotes that no detector is selected.
///
/// `Octavius1500` represents the Octavius 1500 detector.
///
/// # Notes
///
/// - The `Clone`, `Debug`, `Eq`, and `PartialEq` traits are implemented for `DetectorType`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DetectorType {
    None,
    Octavius1500,
}

impl Default for DetectorType {
    fn default() -> Self {
        Self::None
    }
}

impl FromStr for DetectorType {
    type Err = DosimetryToolsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "PTW_DETECTOR_OCTAVIUS_1500" {
            Ok(Self::Octavius1500)
        } else {
            Err(DosimetryToolsError::ParseDetectorTypeError(s.to_string()))
        }
    }
}
