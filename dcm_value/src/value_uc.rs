use crate::{DicomValue, Value};
use dicom_object::InMemDicomObject;

crate::dicom_value_type!(UC, UC, String);
crate::dicom_value_type!(UCs, UC, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(UC, UCs, '\\');
crate::from_dicom_object_for_string!(UC, UC);
crate::from_dicom_object_for_strings!(UCs, UC, '\\');
crate::dicom_value_from_str!(UC);
crate::dicom_value_from_same_type!(UC, String);
crate::dicom_value_from_same_type!(UCs, Vec<String>);
crate::to_dicom_object_for_string!(UC, UC);
crate::to_dicom_object_for_strings!(UCs, UC);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for UC<G, E> {}
impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for UCs<G, E> {}
