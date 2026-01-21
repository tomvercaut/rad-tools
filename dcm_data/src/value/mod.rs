use dicom_core::{Tag, VR};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VM {
    Single,
    Multiple,
}

pub trait Value<T> {

    fn tag(&self) -> Tag;
    fn vr(&self) -> VR;
    fn value(&self) -> &T;
    fn value_mut(&mut self) -> &mut T;
}

pub struct AEValue<const G: u16, const E: u16> {
    value: String,
}

impl<const G: u16, const E: u16> Value<String> for AEValue<G, E> {
    fn tag(&self) -> Tag {
        Tag(G, E)
    }

    fn vr(&self) -> VR {
        VR::AE
    }

    fn value(&self) -> &String {
        &self.value
    }

    fn value_mut(&mut self) -> &mut String {
        &mut self.value
    }
}
pub struct AEValues<const G: u16, const E: u16> {
    value: Vec<String>,
}

impl<const G: u16, const E: u16> Value<Vec<String>> for AEValues<G, E> {
    fn tag(&self) -> Tag {
        Tag(G, E)
    }

    fn vr(&self) -> VR {
        VR::AE
    }

    fn value(&self) -> &Vec<String> {
        &self.value
    }

    fn value_mut(&mut self) -> &mut Vec<String> {
        &mut self.value
    }
}
