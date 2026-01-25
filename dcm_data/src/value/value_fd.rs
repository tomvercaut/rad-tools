crate::dicom_value_type!(FD, FD, f64);
crate::dicom_value_type!(FDs, FD, Vec<f64>);
crate::from_dicom_object_for_number!(FD, FD, to_float64);
crate::from_dicom_object_for_numbers!(FDs, FD, float64_slice);
