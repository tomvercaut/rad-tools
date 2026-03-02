use crate::error::Error;
use crate::{DicomValue, ReadDicomValue, VM, Value, WriteDicomValue};
use dicom_core::DicomValue::Sequence;
use dicom_core::smallvec::SmallVec;
use dicom_core::value::DataSetSequence;
use dicom_core::{Length, Tag, VR};
use dicom_object::InMemDicomObject;

pub struct SQ<const G: u16, const E: u16, T> {
    value: Vec<T>,
}

impl<const G: u16, const E: u16, T> Value<Vec<T>> for SQ<G, E, T> {
    fn tag(&self) -> Tag {
        use std::cell::LazyCell;
        let lt: LazyCell<Tag> = LazyCell::new(|| Tag(G, E));
        *lt
    }

    fn vr(&self) -> VR {
        VR::SQ
    }

    fn vm(&self) -> VM {
        VM::Single
    }

    fn value(&self) -> &Vec<T> {
        &self.value
    }

    fn value_mut(&mut self) -> &mut Vec<T> {
        &mut self.value
    }
}

impl<const G: u16, const E: u16, T> std::ops::Deref for SQ<G, E, T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<const G: u16, const E: u16, T> std::ops::DerefMut for SQ<G, E, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<const G: u16, const E: u16, T> ReadDicomValue<InMemDicomObject> for SQ<G, E, T>
where
    T: ReadDicomValue<InMemDicomObject>,
{
    fn read_value(backend: &InMemDicomObject) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match backend.element(Tag(G, E)) {
            Ok(elem) => {
                if elem.vr() == VR::SQ {
                    match elem.items() {
                        Some(items) => {
                            let mut value = Vec::new();
                            for item in items {
                                value.push(T::read_value(item)?);
                            }
                            Ok(Self { value })
                        }
                        None => Ok(Self { value: Vec::new() }),
                    }
                } else {
                    Err(Error::InvalidVRMatch(VR::SQ, elem.vr()))
                }
            }
            Err(e) => Err(Error::from(e)),
        }
    }
}

impl<const G: u16, const E: u16, T> WriteDicomValue<InMemDicomObject> for SQ<G, E, T>
where
    T: WriteDicomValue<InMemDicomObject>,
{
    fn write_value(&self, backend: &mut InMemDicomObject) -> Result<(), Error> {
        let mut sv = SmallVec::<[InMemDicomObject; 2]>::new();

        for item in &self.value {
            let mut item_obj = InMemDicomObject::new_empty();
            item.write_value(&mut item_obj)?;
            sv.push(item_obj);
        }

        let dss = DataSetSequence::new(sv, Length::UNDEFINED);
        let seq_value = Sequence(dss);

        backend.put(dicom_core::DataElement::new(Tag(G, E), VR::SQ, seq_value));

        Ok(())
    }
}

impl<const G: u16, const E: u16, T> DicomValue<InMemDicomObject> for SQ<G, E, T> where
    T: DicomValue<InMemDicomObject>
{
}
