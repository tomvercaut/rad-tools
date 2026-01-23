crate::dicom_value_type!(LT, LT, String);
crate::dicom_value_type!(LTs, LT, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(LT, LTs, '\\');
crate::from_dicom_object_for_string!(LT, LT);
crate::from_dicom_object_for_strings!(LTs, LT, '\\');
