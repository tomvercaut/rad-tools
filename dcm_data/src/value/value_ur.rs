crate::dicom_value_type!(UR, UR, String);
crate::dicom_value_type!(URs, UR, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(UR, URs, '\\');
crate::from_dicom_object_for_string!(UR, UR);
crate::from_dicom_object_for_strings!(URs, UR, '\\');
