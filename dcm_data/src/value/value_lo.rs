crate::dicom_value_type!(LO, LO, String);
crate::dicom_value_type!(LOs, LO, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(LO, LOs, '\\');
crate::from_dicom_object_for_string!(LO, LO);
crate::from_dicom_object_for_strings!(LOs, LO, '\\');
crate::dicom_value_from_str!(LO);
crate::dicom_value_from_same_type!(LO, String);
crate::dicom_value_from_same_type!(LOs, Vec<String>);
