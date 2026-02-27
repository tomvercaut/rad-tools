use crate::io::DcmIOError;

pub(crate) mod macros;
mod value_ae;
pub use value_ae::*;
mod value_as;
pub use value_as::*;
mod value_at;
pub use value_at::*;
mod value_cs;
pub use value_cs::*;
mod value_da;
pub use value_da::*;
mod value_ds;
pub use value_ds::*;
mod value_dt;
pub use value_dt::*;
mod value_fd;
pub use value_fd::*;
mod value_fl;
pub use value_fl::*;
mod value_is;
pub use value_is::*;
mod value_lo;
pub use value_lo::*;
mod value_lt;
pub use value_lt::*;
mod value_od;
pub use value_od::*;
mod value_of;
pub use value_of::*;
mod value_ov;
pub use value_ov::*;
mod value_ow;
pub use value_ow::*;
mod value_pn;
pub use value_pn::*;
mod value_sh;
pub use value_sh::*;
mod value_sl;
pub use value_sl::*;
mod value_ss;
pub use value_ss::*;
mod value_sv;
pub use value_sv::*;
mod value_st;
pub use value_st::*;
mod value_tm;
pub use value_tm::*;
mod value_uc;
pub use value_uc::*;
mod value_ui;
pub use value_ui::*;
mod value_ul;
pub use value_ul::*;
mod value_ur;
pub use value_ur::*;
mod value_us;
pub use value_us::*;
mod value_ut;
pub use value_ut::*;

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

pub trait FromDicomObject {
    fn from_object(obj: &dicom_object::InMemDicomObject) -> Result<Self, DcmIOError>
    where
        Self: Sized;
    fn from_object_opt(obj: &dicom_object::InMemDicomObject) -> Result<Option<Self>, DcmIOError>
    where
        Self: Sized,
    {
        match Self::from_object(obj) {
            Ok(value) => Ok(Some(value)),
            Err(e) => match e {
                DcmIOError::DicomElementAccessError(_) => Ok(None),
                _ => Err(e),
            },
        }
    }
}

pub trait ToDicomObject {
    fn to_object(&self, obj: &mut dicom_object::InMemDicomObject) -> Result<(), DcmIOError>;
}

crate::dicom_value_type!(OB, OB, Vec<u8>);
crate::dicom_value_type!(UN, UN, Vec<u8>);
