use dicom_object::InMemDicomObject;
use crate::{Value, DicomValue};

crate::dicom_value_type!(UN, UN, Vec<u8>);
crate::from_dicom_object_for_numbers!(UN, UN, uint8_slice);
crate::to_dicom_object_for_numbers!(UN, UN, u8, U8);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for UN<G, E> {}
