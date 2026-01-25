crate::dicom_value_type!(SV, SV, i64);
crate::dicom_value_type!(SVs, SV, Vec<i64>);
crate::from_dicom_object_for_number!(SV, SV, int64);
crate::from_dicom_object_for_numbers!(SVs, SV, int64_slice);

