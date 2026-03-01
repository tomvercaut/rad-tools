use dicom_object::InMemDicomObject;
use crate::{Value, DicomValue};

crate::dicom_value_type!(DS, DS, String);
crate::dicom_value_type!(DSs, DS, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(DS, DSs, '\\');
crate::from_dicom_object_for_string!(DS, DS);
crate::from_dicom_object_for_strings!(DSs, DS, '\\');
crate::dicom_value_from_str!(DS);
crate::dicom_value_from_same_type!(DS, String);
crate::dicom_value_from_same_type!(DSs, Vec<String>);
crate::to_dicom_object_for_string!(DS, DS);
crate::to_dicom_object_for_strings!(DSs, DS);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for DS<G, E> {}
impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for DSs<G, E> {}