use crate::{DicomValue, Value};
use dicom_object::InMemDicomObject;

crate::dicom_value_type!(UL, UL, u32);
crate::dicom_value_type!(ULs, UL, Vec<u32>);
crate::from_dicom_object_for_number!(UL, UL, uint32);
crate::from_dicom_object_for_numbers!(ULs, UL, uint32_slice);
crate::to_dicom_object_for_number!(UL, UL, u32, U32);
crate::to_dicom_object_for_numbers!(ULs, UL, u32, U32);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for UL<G, E> {}
impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for ULs<G, E> {}
