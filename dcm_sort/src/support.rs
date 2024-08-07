use dicom_core::Tag;
use dicom_object::{FileDicomObject, InMemDicomObject};
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub(crate) enum ElementError {
    #[error("Unable to access tag {0} in DICOM object.")]
    AccessError(Tag),
    #[error("Unable to convert DICOM tag [{0}] value to string")]
    StringConvertValue(Tag),
}

pub(crate) type Result<T> = std::result::Result<T, ElementError>;

fn get_str_internal(
    obj: &FileDicomObject<InMemDicomObject>,
    tag: Tag,
    log_errors: bool,
) -> Result<String> {
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

pub(crate) fn get_str(obj: &FileDicomObject<InMemDicomObject>, tag: Tag) -> Result<String> {
    get_str_internal(obj, tag, true)
}

pub(crate) fn get_str_or_default(obj: &FileDicomObject<InMemDicomObject>, tag: Tag) -> String {
    get_str_internal(obj, tag, false).unwrap_or_default()
}
