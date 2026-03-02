use crate::{DicomValue, Value};
use dicom_object::InMemDicomObject;

crate::dicom_value_type!(LO, LO, String);
crate::dicom_value_type!(LOs, LO, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(LO, LOs, '\\');
crate::from_dicom_object_for_string!(LO, LO);
crate::from_dicom_object_for_strings!(LOs, LO, '\\');
crate::dicom_value_from_str!(LO);
crate::dicom_value_from_same_type!(LO, String);
crate::dicom_value_from_same_type!(LOs, Vec<String>);
crate::to_dicom_object_for_string!(LO, LO);
crate::to_dicom_object_for_strings!(LOs, LO);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for LO<G, E> {}
impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for LOs<G, E> {}