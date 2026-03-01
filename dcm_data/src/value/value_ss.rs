use crate::{DicomValue, Value};
use dicom_object::InMemDicomObject;

crate::dicom_value_type!(SS, SS, i16);
crate::dicom_value_type!(SSs, SS, Vec<i16>);
crate::from_dicom_object_for_number!(SS, SS, int16);
crate::from_dicom_object_for_numbers!(SSs, SS, int16_slice);
crate::to_dicom_object_for_number!(SS, SS, i16, I16);
crate::to_dicom_object_for_numbers!(SSs, SS, i16, I16);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for SS<G, E> {}
impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for SSs<G, E> {}
