crate::dicom_value_type!(SL, SL, i32);
crate::dicom_value_type!(SLs, SL, Vec<i32>);
crate::from_dicom_object_for_number!(SL, SL, int32);
crate::from_dicom_object_for_numbers!(SLs, SL, int32_slice);

