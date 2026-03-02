use crate::{DicomValue, Value};
use dicom_object::InMemDicomObject;

crate::dicom_value_type!(UI, UI, String);
crate::dicom_value_type!(UIs, UI, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(UI, UIs, '\\');
crate::from_dicom_object_for_string!(UI, UI);
crate::from_dicom_object_for_strings!(UIs, UI, '\\');
crate::dicom_value_from_str!(UI);
crate::dicom_value_from_same_type!(UI, String);
crate::dicom_value_from_same_type!(UIs, Vec<String>);
crate::to_dicom_object_for_string!(UI, UI);
crate::to_dicom_object_for_strings!(UIs, UI);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for UI<G, E> {}
impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for UIs<G, E> {}
