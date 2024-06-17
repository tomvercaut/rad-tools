use ndarray::Array3;

use crate::data::xcc::Xcc;
use crate::data::{DetectorType, TaskType};
use crate::PeeTeeWeeError;

/// The `Detector` trait represents a type that can be used to detect something.
pub trait Detector {}

/// The `DetectorInterpolation` trait represents a type that can be used to perform
/// interpolation on detector data.
trait DetectorInterpolation<T: Detector> {
    fn interpolate(&self) -> Result<T, PeeTeeWeeError>;
}

/// The `Octavius1500` struct represents a PTW Octavius 1500 device.
///
/// It contains the following fields:
///
/// - `data`: An `Array3<f64>` representing the device's data.
/// - `interpolated`: A boolean indicating if the data has been interpolated.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Octavius1500 {
    data: Array3<f64>,
    pub(crate) interpolated: bool,
}

impl Detector for Octavius1500 {}

impl Octavius1500 {
    /// Creates a new Octavius1500 instance from an Xcc instance.
    ///
    /// # Arguments
    ///
    /// * `xcc` - A reference to an Xcc struct.
    /// * `interpolate` - A flag indicating whether to interpolate the data or not.
    ///
    /// # Returns
    ///
    /// * A Result containing the Octavius1500 instance if successful, or an error if the task type or detector type is invalid.
    pub fn new(xcc: &Xcc, interpolate: bool) -> Result<Octavius1500, PeeTeeWeeError> {
        if xcc.content.administrative.task_name != TaskType::Measurement2dArray {
            return Err(PeeTeeWeeError::Octavius1500FromXccError(
                "invalid task type".to_string(),
            ));
        } else if xcc.content.measuring_device.detector != DetectorType::Octavius1500 {
            return Err(PeeTeeWeeError::Octavius1500FromXccError(
                "invalid detector type type".to_string(),
            ));
        }
        let nr = (xcc.content.detector_array.matrix_number_of_meas_gt * 2) as usize;
        let nc = (xcc.content.detector_array.matrix_number_of_meas_lr * 2) as usize;
        let nd = xcc.content.measurement_data.measurements.len();
        let mut data = Array3::<f64>::zeros((nd, nr, nc));
        let mut r: usize;
        let mut c: usize;
        for d in 0..nd {
            let measurement = &xcc.content.measurement_data.measurements[d];
            r = 0;
            c = 0;
            for value in &measurement.data {
                let pixel = data.get_mut((d, r, c)).unwrap();
                *pixel = *value;
                c += 2;
                if c >= nc {
                    r += 1;
                    if r % 2 != 0 {
                        c = 1;
                    } else {
                        c = 0;
                    }
                }
            }
        }
        if interpolate {
            unimplemented!()
        }
        Ok(Self {
            data,
            interpolated: interpolate,
        })
    }

    /// Returns a reference to the value at the specified position in the 3-dimensional data.
    ///
    /// # Arguments
    ///
    /// * `d` - The index of the measurement.
    /// * `r` - The index of the row dimension.
    /// * `c` - The index of the column dimension.
    ///
    /// # Panics
    ///
    /// Panics if the position is out of bounds.
    ///
    /// # Returns
    ///
    /// A reference to the value located at the specified position.
    fn get_unchecked(&self, d: usize, r: usize, c: usize) -> &f64 {
        self.data.get((d, r, c)).unwrap()
    }

    /// Returns a reference to the value at the specified position in the 3-dimensional data.
    ///
    /// # Arguments
    ///
    /// * `d` - The index of the measurement.
    /// * `r` - The index of the row dimension.
    /// * `c` - The index of the column dimension.
    ///
    /// # Panics
    ///
    /// Panics if the position is out of bounds.
    ///
    /// # Returns
    ///
    /// A reference to the value located at the specified position.
    pub fn get(&self, d: usize, r: usize, c: usize) -> Result<&f64, PeeTeeWeeError> {
        match self.data.get((d, r, c)) {
            None => Err(PeeTeeWeeError::IndexOutOfBound),
            Some(f) => Ok(f),
        }
    }

    /// Sets the value at the specified indices in the three-dimensional data array.
    ///
    /// The function doesn't allocate any memory and only overwrites existing data.
    ///
    /// # Arguments
    ///
    /// - `d`: The index of the measurement.
    /// - `r`: The index of the row dimension.
    /// - `c`: The index of the column dimension.
    /// - `f`: The value to set.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the value was successfully set, or `Err(DosimetryToolsError::IndexOutOfBound)`
    /// if the provided indices are out of bounds.
    pub fn set(&mut self, d: usize, r: usize, c: usize, f: f64) -> Result<(), PeeTeeWeeError> {
        match self.data.get_mut((d, r, c)) {
            None => Err(PeeTeeWeeError::IndexOutOfBound),
            Some(value) => {
                *value = f;
                Ok(())
            }
        }
    }

    pub fn rows(&self) -> usize {
        self.data.shape()[1]
    }

    pub fn cols(&self) -> usize {
        self.data.shape()[2]
    }

    pub fn measurement_len(&self) -> usize {
        self.data.shape()[0]
    }

    pub fn sum(&self) -> Result<Self, PeeTeeWeeError> {
        let nr = self.rows();
        let nc = self.cols();
        let nd = 1;
        let mut oct = Self {
            data: Array3::<f64>::zeros((nd, nr, nc)),
            interpolated: self.interpolated,
        };
        for r in 0..nr {
            for c in 0..nc {
                let mut sum = 0.0;
                for d in 0..nd {
                    sum += self.get(d, r, d)?;
                }
                oct.set(0, r, c, sum)?;
            }
        }
        Ok(oct)
    }
}

pub struct Octavius1500Interpolator {}

impl DetectorInterpolation<Octavius1500> for Octavius1500 {
    fn interpolate(&self) -> Result<Octavius1500, PeeTeeWeeError> {
        let mut oct = self.clone();
        if oct.interpolated {
            return Ok(oct);
        }
        let nr = oct.rows();
        let nc = oct.cols();
        let nd = oct.measurement_len();
        let mut r: usize;
        let mut c: usize;
        for d in 0..nd {
            r = 0;
            c = 1;
            loop {
                let value: f64 = if r == 0 || r == nr - 1 {
                    (*self.get(d, r, c - 1).unwrap() + *self.get(d, r, c + 1).unwrap()) * 0.5
                } else if c == 0 || c == nc - 1 {
                    (*self.get(d, r - 1, c).unwrap() + *self.get(d, r + 1, c).unwrap()) * 0.5
                } else {
                    (*self.get(d, r, c - 1).unwrap()
                        + *self.get(d, r, c + 1).unwrap()
                        + *self.get(d, r - 1, c).unwrap()
                        + *self.get(d, r + 1, c).unwrap())
                        * 0.25
                };
                oct.set(d, r, c, value)?;

                c += 2;
                if c >= nc {
                    r += 1;
                    if r % 2 == 0 {
                        c = 1;
                    } else {
                        c = 0;
                    }
                }
                if r >= nr {
                    break;
                }
            }
        }
        oct.interpolated = true;
        Ok(oct)
    }
}
