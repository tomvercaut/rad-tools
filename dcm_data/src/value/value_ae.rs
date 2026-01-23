crate::dicom_value_type!(AE, AE, String);
crate::dicom_value_type!(AEs, AE, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(AE, AEs, '\\');
crate::from_dicom_object_for_string!(AE, AE);
crate::from_dicom_object_for_strings!(AEs, AE, '\\');
