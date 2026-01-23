crate::dicom_value_type!(CS, CS, String);
crate::dicom_value_type!(CSs, CS, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(CS, CSs, '\\');
crate::from_dicom_object_for_string!(CS, CS);
crate::from_dicom_object_for_strings!(CSs, CS, '\\');
crate::dicom_value_from_str!(CS);
crate::dicom_value_from_same_type!(CS, String);
crate::dicom_value_from_same_type!(CSs, Vec<String>);
