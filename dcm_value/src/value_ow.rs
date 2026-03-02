use crate::{DicomValue, Value};
use dicom_object::InMemDicomObject;

crate::dicom_value_type!(OW, OW, Vec<u16>);
crate::from_dicom_object_for_numbers!(OW, OW, uint16_slice);
crate::to_dicom_object_for_numbers!(OW, OW, u16, U16);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for OW<G, E> {}
