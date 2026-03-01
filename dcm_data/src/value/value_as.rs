use dicom_object::InMemDicomObject;
use crate::{Value, DicomValue};

crate::dicom_value_type!(AS, AS, String);
crate::dicom_value_type!(ASs, AS, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(AS, ASs, '\\');
crate::from_dicom_object_for_string!(AS, AS);
crate::from_dicom_object_for_strings!(ASs, AS, '\\');
crate::dicom_value_from_str!(AS);
crate::dicom_value_from_same_type!(AS, String);
crate::dicom_value_from_same_type!(ASs, Vec<String>);
crate::to_dicom_object_for_string!(AS, AS);
crate::to_dicom_object_for_strings!(ASs, AS);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for AS<G, E> {}
impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for ASs<G, E> {}