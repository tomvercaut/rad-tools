use dicom_core::PrimitiveValue;
use crate::Value;
use crate::io::DcmIOError;
use crate::io::DcmIOError::InvalidVRMatch;
use dicom_object::InMemDicomObject;

crate::dicom_value_type!(Tag, AT, dicom_core::Tag);
crate::dicom_value_type!(Tags, AT, Vec<dicom_core::Tag>);
crate::dicom_value_from_same_type!(Tag, dicom_core::Tag);
crate::dicom_value_from_same_type!(Tags, Vec<dicom_core::Tag>);

impl<const G: u16, const E: u16> crate::value::ReadDicomValue<InMemDicomObject>
for Tag<G, E>
{
    fn read_value(obj: &InMemDicomObject) -> Result<Self, DcmIOError> {
        match obj.element(dicom_core::Tag(G, E)) {
            Ok(elem) => {
                if elem.vr() == dicom_core::VR::AT {
                    let slice = elem.uint16_slice()?;
                    if (slice.len() % 2) != 0 {
                        return Err(DcmIOError::InvalidNumberOfTagValues(
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
            Err(e) => Err(DcmIOError::from(e))?,
        }
    }
}

impl<const G: u16, const E: u16> crate::value::ReadDicomValue<InMemDicomObject>
    for Tags<G, E>
{
    fn read_value(obj: &InMemDicomObject) -> Result<Self, DcmIOError> {
        match obj.element(dicom_core::Tag(G, E)) {
            Ok(elem) => {
                if elem.vr() == dicom_core::VR::AT {
                    let slice = elem.uint16_slice()?;
                    let m = slice.len();
                    if (m % 2) != 0 {
                        return Err(DcmIOError::InvalidNumberOfTagValues(
                            2,
                            m,
                        ));
                    }
                    let mut i = 0;
                    let mut value = vec![];
                    while i < m {
                        let t = dicom_core::Tag(slice[i], slice[i+1]);
                        value.push(t);
                        i+=2;
                    }
                    Ok(Self { value })
                } else {
                    Err(InvalidVRMatch(dicom_core::VR::AT, elem.vr()))
                }
            }
            Err(e) => Err(DcmIOError::from(e))?,
        }
    }
}

impl<const G: u16, const E: u16> crate::value::WriteDicomValue<InMemDicomObject>
for Tag<G, E>
{
    fn write_value(&self, obj: &mut InMemDicomObject) -> Result<(), DcmIOError> {
        let mut p = dicom_core::smallvec::SmallVec::<[dicom_core::Tag;2]>::new();
        p.push(self.value().clone());
        let _ = obj.put(dicom_core::DataElement::new(
            self.tag(),
            self.vr(),
            PrimitiveValue::Tags(p),
        ));
        Ok(())
    }
}

impl<const G: u16, const E: u16> crate::value::WriteDicomValue<InMemDicomObject>
    for Tags<G, E>
{
    fn write_value(&self, obj: &mut InMemDicomObject) -> Result<(), DcmIOError> {
        let mut p = dicom_core::smallvec::SmallVec::<[dicom_core::Tag;2]>::new();
        for t in self.value() {
            p.push(t.clone());
        }
        let _ = obj.put(dicom_core::DataElement::new(
            self.tag(),
            self.vr(),
            PrimitiveValue::Tags(p),
        ));
        Ok(())
    }
}
