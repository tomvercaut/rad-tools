use crate::{DicomValue, Value};
use dicom_object::InMemDicomObject;

crate::dicom_value_type!(ST, ST, String);
crate::dicom_value_type!(STs, ST, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(ST, STs, '\\');
crate::from_dicom_object_for_string!(ST, ST);
crate::from_dicom_object_for_strings!(STs, ST, '\\');
crate::dicom_value_from_str!(ST);
crate::dicom_value_from_same_type!(ST, String);
crate::dicom_value_from_same_type!(STs, Vec<String>);
crate::to_dicom_object_for_string!(ST, ST);
crate::to_dicom_object_for_strings!(STs, ST);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for ST<G, E> {}
impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for STs<G, E> {}