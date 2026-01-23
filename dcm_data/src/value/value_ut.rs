crate::dicom_value_type!(UT, UT, String);
crate::dicom_value_type!(UTs, UT, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(UT, UTs, '\\');
crate::from_dicom_object_for_string!(UT, UT);
crate::from_dicom_object_for_strings!(UTs, UT, '\\');
crate::dicom_value_from_str!(UT);
crate::dicom_value_from_same_type!(UT, String);
crate::dicom_value_from_same_type!(UTs, Vec<String>);
