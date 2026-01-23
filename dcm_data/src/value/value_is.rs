crate::dicom_value_type!(IS, IS, String);
crate::dicom_value_type!(ISs, IS, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(IS, ISs, '\\');
crate::from_dicom_object_for_string!(IS, IS);
crate::from_dicom_object_for_strings!(ISs, IS, '\\');
