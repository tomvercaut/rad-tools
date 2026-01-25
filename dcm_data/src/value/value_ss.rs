crate::dicom_value_type!(SS, SS, i16);
crate::dicom_value_type!(SSs, SS, Vec<i16>);
crate::from_dicom_object_for_number!(SS, SS, int16);
crate::from_dicom_object_for_numbers!(SSs, SS, int16_slice);

