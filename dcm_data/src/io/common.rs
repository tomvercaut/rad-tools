use crate::io::{to_string, DcmIOError};
use crate::Sop;
use dicom_core::Tag;
use dicom_object::InMemDicomObject;


/// Reads SOP (Service-Object Pair) from a DICOM object.
///
/// This function extracts the `class_uid` and `instance_uid` from the provided
/// DICOM object and constructs a `Sop` struct. It returns an error
/// if any of the elements required to construct the `Sop` struct are missing.
///
/// # Arguments
///
/// * `obj` - The DICOM object from which to read the SOP information.
/// * `class_uid` - The tag corresponding to the class UID element.
/// * `instance_uid` - The tag corresponding to the instance UID element.
///
/// # Returns
///
/// Returns a `Result` containing a `Sop` struct on success, or a `DcmIOError` on failure.
///
/// # Errors
///
/// This function will return an error if it fails to read any of the required
/// elements (`class_uid` or `instance_uid`) from the DICOM object.
pub(crate) fn read_sop(obj: &InMemDicomObject, class_uid: Tag, instance_uid: Tag) -> Result<Sop, DcmIOError> {
    Ok(Sop {
        class_uid: to_string(&obj, class_uid)?,
        instance_uid: to_string(&obj, instance_uid)?,
    })
}


///
/// Reads optional SOP (Service-Object Pair) from a DICOM object.
///
/// This function extracts the `class_uid` and `instance_uid` from the provided 
/// DICOM object and constructs an optional `Sop` struct. If either element 
/// is missing in the DICOM object, it will return `Ok(None)`. Otherwise, 
/// it will return `Ok(Some(Sop))`.
///
/// # Arguments
///
/// * `obj` - The DICOM object from which to read the SOP information.
/// * `class_uid` - The tag corresponding to the class UID element.
/// * `instance_uid` - The tag corresponding to the instance UID element.
///
/// # Returns
///
/// Returns a `Result` containing an `Option<Sop>`:
/// * `Ok(Some(Sop))` if both `class_uid` and `instance_uid` elements are found in the DICOM object.
/// * `Ok(None)` if either element is missing.
/// * `Err(DcmIOError)` if any error occurs while trying to read the elements.
///
/// # Errors
///
/// This function will return an error if it fails to read any of the required
/// elements (`class_uid` or `instance_uid`) from the DICOM object.
pub(crate) fn read_sop_opt(obj: &InMemDicomObject, class_uid: Tag, instance_uid: Tag) -> Result<Option<Sop>, DcmIOError> {
    match obj.element_opt(class_uid)? {
        None => {Ok(None)}
        Some(class_uid_elem) => {
            match obj.element_opt(instance_uid)? {
                None => {Ok(None)}
                Some(instance_uid_elem) => {
                    Ok(Some(Sop {
                        class_uid: class_uid_elem.to_str()?.to_string(),
                        instance_uid: instance_uid_elem.to_str()?.to_string(),
                    }))
                }
            }
        }
    }

}