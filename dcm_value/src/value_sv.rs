use crate::{DicomValue, Value};
use dicom_object::InMemDicomObject;

crate::dicom_value_type!(SV, SV, i64);
crate::dicom_value_type!(SVs, SV, Vec<i64>);
crate::from_dicom_object_for_number!(SV, SV, int64);
crate::from_dicom_object_for_numbers!(SVs, SV, int64_slice);
crate::to_dicom_object_for_number!(SV, SV, i64, I64);
crate::to_dicom_object_for_numbers!(SVs, SV, i64, I64);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for SV<G, E> {}
impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for SVs<G, E> {}

