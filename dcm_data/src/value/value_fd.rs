use dicom_object::InMemDicomObject;
use crate::{Value, DicomValue};

crate::dicom_value_type!(FD, FD, f64);
crate::dicom_value_type!(FDs, FD, Vec<f64>);
crate::from_dicom_object_for_number!(FD, FD, to_float64);
crate::from_dicom_object_for_numbers!(FDs, FD, float64_slice);
crate::to_dicom_object_for_number!(FD, FD, f64, F64);
crate::to_dicom_object_for_numbers!(FDs, FD, f64, F64);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for FD<G, E> {}
impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for FDs<G, E> {}
