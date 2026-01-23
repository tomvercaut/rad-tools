crate::dicom_value_type!(AE, AE, String);
crate::dicom_value_type!(AEs, AE, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(AE, AEs, '\\');
crate::from_dicom_object_for_string!(AE, AE);
crate::from_dicom_object_for_strings!(AEs, AE, '\\');
crate::dicom_value_from_str!(AE);
crate::dicom_value_from_same_type!(AE, String);
crate::dicom_value_from_same_type!(AEs, Vec<String>);
