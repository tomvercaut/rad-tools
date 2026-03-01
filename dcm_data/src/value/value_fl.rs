use dicom_object::InMemDicomObject;
use crate::{Value, DicomValue};

crate::dicom_value_type!(FL, FL, f32);
crate::dicom_value_type!(FLs, FL, Vec<f32>);
crate::from_dicom_object_for_number!(FL, FL, to_float32);
crate::from_dicom_object_for_numbers!(FLs, FL, float32_slice);
crate::to_dicom_object_for_number!(FL, FL, f32, F32);
crate::to_dicom_object_for_numbers!(FLs, FL, f32, F32);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for FL<G, E> {}
impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for FLs<G, E> {}
