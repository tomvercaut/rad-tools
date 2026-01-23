crate::dicom_value_type!(IS, IS, String);
crate::dicom_value_type!(ISs, IS, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(IS, ISs, '\\');
crate::from_dicom_object_for_string!(IS, IS);
crate::from_dicom_object_for_strings!(ISs, IS, '\\');
crate::dicom_value_from_str!(IS);
crate::dicom_value_from_same_type!(IS, String);
crate::dicom_value_from_same_type!(ISs, Vec<String>);