use crate::PersonName;

mod macros;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VM {
    Single,
    Multiple,
}

pub trait Value<T> {
    fn tag(&self) -> dicom_core::Tag;
    fn vr(&self) -> dicom_core::VR;
    fn vm(&self) -> VM;
    fn value(&self) -> &T;
    fn value_mut(&mut self) -> &mut T;
}

crate::dicom_value_type!(AE, AE, String);
crate::dicom_value_type!(AEs, AE, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(AE, AEs, '\\');
crate::dicom_value_type!(AS, AS, String);
crate::dicom_value_type!(ASs, AS, Vec<String>);
crate::dicom_value_type!(Tag, AT, dicom_core::Tag);
crate::dicom_value_type!(Tags, AT, Vec<dicom_core::Tag>);
crate::dicom_value_type!(CS, CS, String);
crate::dicom_value_type!(CSs, CS, Vec<String>);
crate::dicom_value_type!(DA, DA, chrono::NaiveDate);
crate::dicom_value_type!(DAs, DA, Vec<chrono::NaiveDate>);
crate::dicom_value_type!(DS, DS, String);
crate::dicom_value_type!(DSs, DS, Vec<String>);
crate::dicom_value_type!(DT, DT, chrono::NaiveDateTime);
crate::dicom_value_type!(DTs, DT, Vec<chrono::NaiveDateTime>);
crate::dicom_value_type!(FL, FL, f32);
crate::dicom_value_type!(FLs, FL, Vec<f32>);
crate::dicom_value_type!(FD, FD, f64);
crate::dicom_value_type!(FDs, FD, Vec<f64>);
crate::dicom_value_type!(IS, IS, String);
crate::dicom_value_type!(ISs, IS, Vec<String>);
crate::dicom_value_type!(LO, LO, String);
crate::dicom_value_type!(LOs, LO, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(LO, LOs, '\\');
crate::dicom_value_type!(LT, LT, String);
crate::dicom_value_type!(LTs, LT, Vec<String>);
crate::dicom_value_type!(OB, OB, Vec<u8>);
crate::dicom_value_type!(OD, OD, Vec<f64>);
crate::dicom_value_type!(OF, OF, Vec<f32>);
crate::dicom_value_type!(OV, OV, Vec<u64>);
crate::dicom_value_type!(OW, OW, Vec<u16>);
crate::dicom_value_type!(PN, PN, PersonName);
crate::dicom_value_type!(PNs, PN, Vec<PersonName>);
crate::dicom_value_type!(SH, SH, String);
crate::dicom_value_type!(SHs, SH, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(SH, SHs, '\\');
crate::dicom_value_type!(SL, SL, i32);
crate::dicom_value_type!(SLs, SL, Vec<i32>);
crate::dicom_value_type!(SS, SS, i16);
crate::dicom_value_type!(SSs, SS, Vec<i16>);
crate::dicom_value_type!(ST, ST, String);
crate::dicom_value_type!(STs, ST, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(ST, STs, '\\');
crate::dicom_value_type!(SV, SV, i64);
crate::dicom_value_type!(SVs, SV, Vec<i64>);
crate::dicom_value_type!(TM, TM, chrono::NaiveTime);
crate::dicom_value_type!(TMs, TM, Vec<chrono::NaiveTime>);
crate::dicom_value_type!(UC, UC, String);
crate::dicom_value_type!(UCs, UC, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(UC, UCs, '\\');
crate::dicom_value_type!(UI, UI, String);
crate::dicom_value_type!(UIs, UI, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(UI, UIs, '\\');
crate::dicom_value_type!(UL, UL, u32);
crate::dicom_value_type!(ULs, UL, Vec<u32>);
crate::dicom_value_type!(UN, UN, Vec<u8>);
crate::dicom_value_type!(UR, UR, String);
crate::dicom_value_type!(URs, UR, Vec<String>);
crate::dicom_value_type!(US, US, u16);
crate::dicom_value_type!(USs, US, Vec<u16>);
crate::dicom_value_type!(UT, UT, String);
crate::dicom_value_type!(UTs, UT, Vec<String>);
