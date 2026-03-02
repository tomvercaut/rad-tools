use crate::{DicomValue, Value};
use dicom_object::InMemDicomObject;

crate::dicom_value_type!(LT, LT, String);
crate::dicom_value_type!(LTs, LT, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(LT, LTs, '\\');
crate::from_dicom_object_for_string!(LT, LT);
crate::from_dicom_object_for_strings!(LTs, LT, '\\');
crate::dicom_value_from_str!(LT);
crate::dicom_value_from_same_type!(LT, String);
crate::dicom_value_from_same_type!(LTs, Vec<String>);
crate::to_dicom_object_for_string!(LT, LT);
crate::to_dicom_object_for_strings!(LTs, LT);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for LT<G, E> {}
impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for LTs<G, E> {}