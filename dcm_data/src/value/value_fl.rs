crate::dicom_value_type!(FL, FL, f32);
crate::dicom_value_type!(FLs, FL, Vec<f32>);
crate::from_dicom_object_for_number!(FL, FL, to_float32);
crate::from_dicom_object_for_numbers!(FLs, FL, float32_slice);
