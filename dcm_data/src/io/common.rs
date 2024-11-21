use crate::io::{from_seq_opt, to_date_opt, to_string, to_string_opt, to_time_opt, DcmIOError};
use crate::{CodeItem, Sop};
use dicom_core::Tag;
use dicom_dictionary_std::tags::{
    CODE_MEANING, CODE_VALUE, CODING_SCHEME_DESIGNATOR, CODING_SCHEME_VERSION,
    CONTEXT_GROUP_EXTENSION_CREATOR_UID, CONTEXT_GROUP_EXTENSION_FLAG, CONTEXT_GROUP_LOCAL_VERSION,
    CONTEXT_GROUP_VERSION, CONTEXT_IDENTIFIER, CONTEXT_UID, EQUIVALENT_CODE_SEQUENCE,
    LONG_CODE_VALUE, MAPPING_RESOURCE_NAME, MAPPING_RESOURCE_UID, REFERENCED_SOP_CLASS_UID,
    REFERENCED_SOP_INSTANCE_UID, TREATMENT_SITE_MODIFIER_CODE_SEQUENCE, URN_CODE_VALUE,
};
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
pub(crate) fn read_sop(
    obj: &InMemDicomObject,
    class_uid: Tag,
    instance_uid: Tag,
) -> Result<Sop, DcmIOError> {
    Ok(Sop {
        class_uid: to_string(obj, class_uid)?,
        instance_uid: to_string(obj, instance_uid)?,
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
pub(crate) fn read_sop_opt(
    obj: &InMemDicomObject,
    class_uid: Tag,
    instance_uid: Tag,
) -> Result<Option<Sop>, DcmIOError> {
    match obj.element_opt(class_uid)? {
        None => Ok(None),
        Some(class_uid_elem) => match obj.element_opt(instance_uid)? {
            None => Ok(None),
            Some(instance_uid_elem) => Ok(Some(Sop {
                class_uid: class_uid_elem.to_str()?.to_string(),
                instance_uid: instance_uid_elem.to_str()?.to_string(),
            })),
        },
    }
}
pub(crate) fn referenced_sop(item: &InMemDicomObject) -> Result<Sop, DcmIOError> {
    read_sop(item, REFERENCED_SOP_CLASS_UID, REFERENCED_SOP_INSTANCE_UID)
}

///
/// Constructs a `CodeItem` from the provided `InMemDicomObject` reference.
///
/// This function extracts various code-related elements from the provided
/// DICOM object and constructs a `CodeItem` struct. It returns an error
/// if any of the required elements cannot be read successfully.
///
/// # Arguments
///
/// * `item` - A reference to an `InMemDicomObject` from which the code information
///            is to be extracted.
///
/// # Returns
///
/// Returns a `Result` containing a `CodeItem` struct on success, or a `DcmIOError` on failure.
///
/// # Errors
///
/// This function will return an error if it fails to read any required elements
/// from the DICOM object.
pub(crate) fn code_item(item: &InMemDicomObject) -> Result<CodeItem, DcmIOError> {
    Ok(CodeItem {
        code_value: to_string_opt(item, CODE_VALUE)?,
        coding_scheme_designator: to_string_opt(item, CODING_SCHEME_DESIGNATOR)?,
        coding_scheme_version: to_string_opt(item, CODING_SCHEME_VERSION)?,
        code_meaning: to_string(item, CODE_MEANING)?,
        context_group_version: to_date_opt(item, CONTEXT_GROUP_VERSION)?,
        context_group_local_version: to_time_opt(item, CONTEXT_GROUP_LOCAL_VERSION)?,
        context_group_extension_flag: to_string_opt(item, CONTEXT_GROUP_EXTENSION_FLAG)?,
        context_group_extension_creator_uid: to_string_opt(
            item,
            CONTEXT_GROUP_EXTENSION_CREATOR_UID,
        )?,
        context_identifier: to_string_opt(item, CONTEXT_IDENTIFIER)?,
        context_uid: to_string_opt(item, CONTEXT_UID)?,
        mapping_resource_uid: to_string_opt(item, MAPPING_RESOURCE_UID)?,
        long_code_value: to_string_opt(item, LONG_CODE_VALUE)?,
        urn_code_value: to_string_opt(item, URN_CODE_VALUE)?,
        equivalent_code_sequence: from_seq_opt(item, EQUIVALENT_CODE_SEQUENCE, code_item)?,
        mapping_resource_name: to_string_opt(item, MAPPING_RESOURCE_NAME)?,
        treatment_site_modifier_code_sequence: from_seq_opt(
            item,
            TREATMENT_SITE_MODIFIER_CODE_SEQUENCE,
            code_item,
        )?,
    })
}
