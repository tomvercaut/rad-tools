use crate::{DicomValue, Value};
use dicom_object::InMemDicomObject;

crate::dicom_value_type!(OD, OD, Vec<f64>);
crate::from_dicom_object_for_numbers!(OD, OD, float64_slice);
crate::to_dicom_object_for_numbers!(OD, OD, f64, F64);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for OD<G, E> {}