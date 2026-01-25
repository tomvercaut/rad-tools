crate::dicom_value_type!(UL, UL, u32);
crate::dicom_value_type!(ULs, UL, Vec<u32>);
crate::from_dicom_object_for_number!(UL, UL, uint32);
crate::from_dicom_object_for_numbers!(ULs, UL, uint32_slice);

