crate::dicom_value_type!(DS, DS, String);
crate::dicom_value_type!(DSs, DS, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(DS, DSs, '\\');
crate::from_dicom_object_for_string!(DS, DS);
crate::from_dicom_object_for_strings!(DSs, DS, '\\');
