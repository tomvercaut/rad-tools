crate::dicom_value_type!(US, US, u16);
crate::dicom_value_type!(USs, US, Vec<u16>);
crate::from_dicom_object_for_number!(US, US, uint16);
crate::from_dicom_object_for_numbers!(USs, US, uint16_slice);

