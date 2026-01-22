mod macros;

use dicom_core::{Tag, VR};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VM {
    Single,
    Multiple,
}

pub trait Value<T> {
    fn tag(&self) -> Tag;
    fn vr(&self) -> VR;
    fn vm(&self) -> VM;
    fn value(&self) -> &T;
    fn value_mut(&mut self) -> &mut T;
}

crate::dicom_value_type!(ValueAE, ValueAEs, AE, String);
crate::dicom_value_type!(ValueAS, ValueASs, AS, String);
crate::dicom_value_type!(ValueTag, ValueTags, AT, Tag);
crate::dicom_value_type!(ValueCS, ValueCSs, CS, Tag);
crate::dicom_value_type!(ValueDA, ValueDAs, DA, String);
crate::dicom_value_type!(ValueDS, ValueDSs, DA, chrono::NaiveDate);
crate::dicom_value_type!(ValueDT, ValueDTs, DT, chrono::NaiveTime);
