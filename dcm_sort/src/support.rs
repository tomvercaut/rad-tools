use dicom_core::Tag;
use dicom_object::InMemDicomObject;
use tracing::error;

#[derive(thiserror::Error, Debug, PartialEq)]
pub(crate) enum ElementError {
    #[error("Unable to access tag {0} in DICOM object.")]
    AccessError(Tag),
    #[error("Unable to convert DICOM tag [{0}] value to string")]
    StringConvertValue(Tag),
}

pub(crate) type Result<T> = std::result::Result<T, ElementError>;

fn get_str_internal(obj: &InMemDicomObject, tag: Tag, log_errors: bool) -> Result<String> {
    let r = obj.element(tag);
    if r.as_ref().is_err() {
        if log_errors {
            error!("{:#?}", r.as_ref().unwrap_err());
        }
        return Err(ElementError::AccessError(tag));
    }
    let elem = r.unwrap();
    let r = elem.to_str();
    if r.as_ref().is_err() {
        if log_errors {
            error!("{:#?}", r.as_ref().unwrap_err());
        }
        return Err(ElementError::StringConvertValue(tag));
    }
    let elem_value = r.unwrap();
    Ok(elem_value.trim().to_string())
}

pub(crate) fn get_str(obj: &InMemDicomObject, tag: Tag) -> Result<String> {
    get_str_internal(obj, tag, true)
}

pub(crate) fn get_str_or_default(obj: &InMemDicomObject, tag: Tag) -> String {
    get_str_internal(obj, tag, false).unwrap_or_default()
}

#[cfg(test)]
mod test {
    use dicom_core::DataElement;
    use dicom_dictionary_std::tags::{PATIENT_ID, PATIENT_NAME};
    use dicom_object::InMemDicomObject;

    use crate::support::ElementError;

    fn get_test_data() -> InMemDicomObject {
        let mut obj = InMemDicomObject::new_empty();
        let pt_id = DataElement::new(PATIENT_ID, dicom_core::VR::LO, "X01");
        obj.put(pt_id);
        obj
    }

    #[test]
    fn get_str_ok() {
        let obj = get_test_data();
        let r = super::get_str(&obj, PATIENT_ID);
        assert!(r.is_ok());
        assert_eq!("X01", r.unwrap().as_str());
    }

    #[test]
    fn get_str_err() {
        let obj = get_test_data();
        let r = super::get_str(&obj, PATIENT_NAME);
        assert!(r.is_err());
        let e = r.unwrap_err();
        assert_eq!(ElementError::AccessError(PATIENT_NAME), e);
    }

    #[test]
    fn get_str_or_default_ok() {
        let obj = get_test_data();
        let s = super::get_str_or_default(&obj, PATIENT_ID);
        assert_eq!("X01", s.as_str());
    }

    #[test]
    fn get_str_or_default_returns_default() {
        let obj = get_test_data();
        let s = super::get_str_or_default(&obj, PATIENT_NAME);
        assert_eq!("", s.as_str());
    }
}
