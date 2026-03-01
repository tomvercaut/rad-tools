use crate::{DicomValue, Value};
use dicom_object::InMemDicomObject;

crate::dicom_value_type!(US, US, u16);
crate::dicom_value_type!(USs, US, Vec<u16>);
crate::from_dicom_object_for_number!(US, US, uint16);
crate::from_dicom_object_for_numbers!(USs, US, uint16_slice);
crate::to_dicom_object_for_number!(US, US, u16, U16);
crate::to_dicom_object_for_numbers!(USs, US, u16, U16);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for US<G, E> {}
impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for USs<G, E> {}
