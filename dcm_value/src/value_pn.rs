use crate::Error;
use crate::{DicomValue, ReadDicomValue, Value};
use crate::{PersonName, WriteDicomValue};
use dicom_core::smallvec::SmallVec;
use dicom_core::PrimitiveValue;
use dicom_object::InMemDicomObject;
use std::str::FromStr;

crate::dicom_value_type!(PN, PN, PersonName);
crate::dicom_value_type!(PNs, PN, Vec<PersonName>);

impl<const G: u16, const E: u16> FromStr for PN<G, E> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = PersonName::from_str(s)?;
        Ok(PN { value })
    }
}

impl<const G: u16, const E: u16> ReadDicomValue<dicom_object::InMemDicomObject> for PN<G, E> {
    fn read_value(backend: &InMemDicomObject) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match backend.element(dicom_core::Tag(G, E)) {
            Ok(elem) => {
                if elem.vr() == dicom_core::VR::PN {
                    let s = elem.string()?;
                    match PersonName::from_str(&s) {
                        Ok(pn) => Ok(PN { value: pn }),
                        Err(_) => Err(Error::InvalidPersonNameFormat(s.to_string())),
                    }
                } else {
                    Err(Error::InvalidVRMatch(dicom_core::VR::PN, elem.vr()))
                }
            }
            Err(e) => Err(Error::from(e))?,
        }
    }
}

impl<const G: u16, const E: u16> ReadDicomValue<dicom_object::InMemDicomObject> for PNs<G, E> {
    fn read_value(backend: &InMemDicomObject) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match backend.element(dicom_core::Tag(G, E)) {
            Ok(elem) => {
                if elem.vr() == dicom_core::VR::PN {
                    let es = elem.strings()?;
                    let mut v = PNs { value: vec![] };
                    for t in es {
                        match PersonName::from_str(&t) {
                            Ok(pn) => {
                                v.value.push(pn);
                                },
                            Err(_) => {
                                return Err(Error::InvalidPersonNameFormat(t.to_string()))
                            }
                        }
                    }
                    Ok(v)
                } else {
                    Err(Error::InvalidVRMatch(dicom_core::VR::PN, elem.vr()))
                }
            }
            Err(e) => Err(Error::from(e))?,
        }
    }
}

impl<const G: u16, const E: u16> WriteDicomValue<dicom_object::InMemDicomObject> for PN<G, E> {
    fn write_value(&self, backend: &mut dicom_object::InMemDicomObject) -> Result<(), Error> {
        let s = self.value().to_string();
        let _ = backend.put(dicom_core::DataElement::new(
            self.tag(),
            self.vr(),
            s.as_str(),
        ));
        Ok(())
    }
}

impl<const G: u16, const E: u16> WriteDicomValue<dicom_object::InMemDicomObject> for PNs<G, E> {
    fn write_value(&self, backend: &mut dicom_object::InMemDicomObject) -> Result<(), Error> {
        let mut sv = SmallVec::<[String; 2]>::new();
        for pn in self.value() {
            sv.push(pn.to_string());
        }
        let pv = PrimitiveValue::Strs(sv);
        let _ = backend.put(dicom_core::DataElement::new(self.tag(), self.vr(), pv));
        Ok(())
    }
}

impl<const G: u16, const E: u16> DicomValue<dicom_object::InMemDicomObject> for PN<G, E> {}
impl<const G: u16, const E: u16> DicomValue<dicom_object::InMemDicomObject> for PNs<G, E> {}