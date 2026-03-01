use crate::{Value, DicomValue};
use dicom_object::InMemDicomObject;

crate::dicom_value_type!(OF, OF, Vec<f32>);
crate::from_dicom_object_for_numbers!(OF, OF, float32_slice);
crate::to_dicom_object_for_numbers!(OF, OF, f32, F32);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for OF<G, E> {}