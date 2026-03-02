use crate::{DicomValue, Value};
use dicom_object::InMemDicomObject;

crate::dicom_value_type!(UR, UR, String);
crate::dicom_value_type!(URs, UR, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(UR, URs, '\\');
crate::from_dicom_object_for_string!(UR, UR);
crate::from_dicom_object_for_strings!(URs, UR, '\\');
crate::dicom_value_from_str!(UR);
crate::dicom_value_from_same_type!(UR, String);
crate::dicom_value_from_same_type!(URs, Vec<String>);
crate::to_dicom_object_for_string!(UR, UR);
crate::to_dicom_object_for_strings!(URs, UR);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for UR<G, E> {}
impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for URs<G, E> {}
