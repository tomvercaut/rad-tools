crate::dicom_value_type!(UC, UC, String);
crate::dicom_value_type!(UCs, UC, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(UC, UCs, '\\');
crate::from_dicom_object_for_string!(UC, UC);
crate::from_dicom_object_for_strings!(UCs, UC, '\\');
