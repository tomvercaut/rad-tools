crate::dicom_value_type!(ST, ST, String);
crate::dicom_value_type!(STs, ST, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(ST, STs, '\\');
crate::from_dicom_object_for_string!(ST, ST);
crate::from_dicom_object_for_strings!(STs, ST, '\\');
