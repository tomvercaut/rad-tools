use crate::{DicomValue, Value};
use dicom_object::InMemDicomObject;

crate::dicom_value_type!(OV, OV, Vec<u64>);
crate::from_dicom_object_for_numbers!(OV, OV, uint64_slice);
crate::to_dicom_object_for_numbers!(OV, OV, u64, U64);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for OV<G, E> {}
