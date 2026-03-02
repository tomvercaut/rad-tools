use crate::{DicomValue, Value};
use dicom_object::InMemDicomObject;

crate::dicom_value_type!(SL, SL, i32);
crate::dicom_value_type!(SLs, SL, Vec<i32>);
crate::from_dicom_object_for_number!(SL, SL, int32);
crate::from_dicom_object_for_numbers!(SLs, SL, int32_slice);
crate::to_dicom_object_for_number!(SL, SL, i32, I32);
crate::to_dicom_object_for_numbers!(SLs, SL, i32, I32);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for SL<G, E> {}
impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for SLs<G, E> {}
