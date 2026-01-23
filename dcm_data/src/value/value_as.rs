crate::dicom_value_type!(AS, AS, String);
crate::dicom_value_type!(ASs, AS, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(AS, ASs, '\\');
crate::from_dicom_object_for_string!(AS, AS);
crate::from_dicom_object_for_strings!(ASs, AS, '\\');
