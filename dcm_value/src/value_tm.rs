use crate::{DicomValue, Value};
use chrono::NaiveTime;
use dicom_object::InMemDicomObject;

crate::dicom_value_type!(TM, TM, String);
crate::dicom_value_type!(TMs, TM, Vec<String>);
crate::dicom_value_from_str!(TM);
crate::from_dicom_object_for_string!(TM, TM);
crate::from_dicom_object_for_strings!(TMs, TM, '\\');
crate::dicom_value_from_same_type!(TM, String);
crate::dicom_value_from_same_type!(TMs, Vec<String>);
crate::to_dicom_object_for_string!(TM, TM);
crate::to_dicom_object_for_strings!(TMs, TM);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for TM<G, E> {}
impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for TMs<G, E> {}

const TIME_FORMATS: [&'static str; 2] = ["%H%M%S", "%H%M%S.%f"];

fn to_time(s: &str) -> Result<Option<NaiveTime>, chrono::ParseError> {
    let mut last_error = None;
    for format in TIME_FORMATS {
        match NaiveTime::parse_from_str(s, format) {
            Ok(value) => {
                return Ok(Some(value));
            }
            Err(e) => {
                last_error = Some(e);
            }
        }
    }
    if let Some(e) = last_error {
        Err(e)
    } else {
        unreachable!();
    }
}

impl<const G: u16, const E: u16> TM<G, E> {
    pub fn to_time(&self) -> Result<Option<NaiveTime>, chrono::ParseError> {
        if self.value.is_empty() {
            return Ok(None);
        }
        to_time(&self.value)
    }
}

impl<const G: u16, const E: u16> TMs<G, E> {
    pub fn to_times(&self) -> Result<Option<Vec<NaiveTime>>, chrono::ParseError> {
        if self.value.is_empty() {
            return Ok(None);
        }
        let mut v = vec![];
        for s in &self.value {
            match to_time(s) {
                Ok(value) => {
                    if let Some(value) = value {
                        v.push(value);
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(Some(v))
    }
}
