crate::dicom_value_type!(SH, SH, String);
crate::dicom_value_type!(SHs, SH, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(SH, SHs, '\\');
crate::from_dicom_object_for_string!(SH, SH);
crate::from_dicom_object_for_strings!(SHs, SH, '\\');
