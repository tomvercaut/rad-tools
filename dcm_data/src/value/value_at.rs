use crate::io::DcmIOError::InvalidVRMatch;

crate::dicom_value_type!(Tag, AT, dicom_core::Tag);
crate::dicom_value_type!(Tags, AT, Vec<dicom_core::Tag>);

impl<const G: u16, const E: u16> crate::value::FromDicomObject for Tag<G, E> {
    fn from_object(obj: &dicom_object::InMemDicomObject) -> Result<Self, crate::io::DcmIOError> {
        match obj.element(dicom_core::Tag(G, E)) {
            Ok(elem) => {
                if elem.vr() == dicom_core::VR::AT {
                    let slice = elem.uint16_slice()?;
                    if (slice.len() % 2) != 0 {
                        return Err(crate::io::DcmIOError::InvalidNumberOfTagValues(
                            2,
                            slice.len(),
                        ));
                    }
                    let value = dicom_core::Tag(slice[0], slice[1]);
                    Ok(Self { value })
                } else {
                    Err(InvalidVRMatch(dicom_core::VR::AT, elem.vr()))
                }
            }
            Err(e) => Err(crate::io::DcmIOError::from(e))?,
        }
    }
}
