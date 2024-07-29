use std::path::Path;

use dicom_core::{Tag, VR};
use dicom_dictionary_std::tags::{
    CONTOUR_IMAGE_SEQUENCE, FRAME_OF_REFERENCE_UID, PATIENT_ID, PATIENT_NAME, PIXEL_DATA,
    REFERENCED_FRAME_OF_REFERENCE_SEQUENCE, REFERENCED_RT_PLAN_SEQUENCE, REFERENCED_SOP_CLASS_UID,
    REFERENCED_SOP_INSTANCE_UID, REFERENCED_STRUCTURE_SET_SEQUENCE, RT_PLAN_LABEL, RT_PLAN_NAME,
    RT_REFERENCED_SERIES_SEQUENCE, RT_REFERENCED_STUDY_SEQUENCE, SERIES_INSTANCE_UID,
    SOP_CLASS_UID, SOP_INSTANCE_UID,
};
use dicom_dictionary_std::uids::{
    CT_IMAGE_STORAGE, ENHANCED_CT_IMAGE_STORAGE, MR_IMAGE_STORAGE,
    POSITRON_EMISSION_TOMOGRAPHY_IMAGE_STORAGE, RT_DOSE_STORAGE, RT_PLAN_STORAGE,
    RT_STRUCTURE_SET_STORAGE,
};
use dicom_object::mem::InMemElement;
use dicom_object::{DefaultDicomObject, InMemDicomObject, OpenFileOptions};
use tracing::trace;

use crate::model::{
    DicomFile, FileInfo, Image, Modality, RTDose, RTPlan, RTReferencedSerie, RTReferencedStudy,
    RTStruct, ReferencedFrameOfReference, ReferencedSopClass, SopClass,
};
use crate::DicomError;
use crate::DicomError::UnsupportedSOPClassUIDReader;

/// Reads a DICOM file partially and returns a `DicomFile` object.
///
/// # Arguments
///
/// * `p` - The path of the DICOM file.
///
/// # Returns
///
/// * `Result<DicomFile, DicomError>` - The `DicomFile` if the file is successfully read, or a `DicomError` if an error occurs.
pub fn read_dicom_file_partial<P: AsRef<Path>>(p: P) -> Result<DicomFile, DicomError> {
    let modalities = vec![
        Modality::Ct,
        Modality::EnhancedCt,
        Modality::Mr,
        Modality::Pt,
        Modality::RtStruct,
        Modality::RtPlan,
        Modality::RtDose,
    ];
    read_dicom_file_partial_by_modalities(p, &modalities)
}

/// Reads a DICOM file partially based on the given modalities.
///
/// # Arguments
///
/// * `p` - The path of the DICOM file to read.
/// * `modalities` - A vector of modalities to filter the DICOM file by.
///
/// # Returns
///
/// * `Result<DicomFile, DicomError>` - A result indicating either a success with the read DICOM file or an error.
pub fn read_dicom_file_partial_by_modalities<P: AsRef<Path>>(
    p: P,
    modalities: &[Modality],
) -> Result<DicomFile, DicomError> {
    let p = p.as_ref();
    trace!("Reading DICOM file: {:#?}", &p);
    let obj = OpenFileOptions::new().read_until(PIXEL_DATA).open_file(p)?;
    let sop_class_uid = get_string(&obj, SOP_CLASS_UID)?;
    if (sop_class_uid == CT_IMAGE_STORAGE && modalities.contains(&Modality::Ct))
        || (sop_class_uid == ENHANCED_CT_IMAGE_STORAGE
            && modalities.contains(&Modality::EnhancedCt))
        || (sop_class_uid == MR_IMAGE_STORAGE && modalities.contains(&Modality::Mr)
            || sop_class_uid == POSITRON_EMISSION_TOMOGRAPHY_IMAGE_STORAGE
                && modalities.contains(&Modality::Pt))
    {
        read_image(p, &obj)
    } else if sop_class_uid == RT_STRUCTURE_SET_STORAGE && modalities.contains(&Modality::RtStruct)
    {
        read_rtstruct(p, &obj)
    } else if sop_class_uid == RT_PLAN_STORAGE && modalities.contains(&Modality::RtPlan) {
        read_rtplan(p, &obj)
    } else if sop_class_uid == RT_DOSE_STORAGE && modalities.contains(&Modality::RtDose) {
        read_rtdose(p, &obj)
    } else {
        Err(UnsupportedSOPClassUIDReader(sop_class_uid.to_string()))
    }
}

/// Read a DICOM image file and create a `DicomFile` with the extracted information.
///
/// # Arguments
///
/// * `path` - A path to the DICOM image file. This should implement the `AsRef<Path>` trait.
/// * `p1` - The default DICOM object representing the DICOM file.
/// * `sop_class_uid` - The SOP class UID of the DICOM file.
///
/// # Returns
///
/// Returns a `Result` that contains a `DicomFile` if successful, or a `DicomError` if an error occurs.
///
/// # Errors
///
/// This function can return a `DicomError` in the following scenarios:
///
/// * If there is an error reading the last modified time of the file.
/// * If there is an error reading the SOP class UID from the default DICOM object.
/// * If there is an error converting the patient ID, patient name, study instance UID, series instance UID, or frame of reference UID to a string.
fn read_image<P: AsRef<Path>>(path: P, obj: &DefaultDicomObject) -> Result<DicomFile, DicomError> {
    let file_info = last_modified_time(path)?;
    let sop = sop_class(obj)?;

    let patient_id = get_string(obj, PATIENT_ID)?;
    let patient_name = get_string(obj, PATIENT_NAME)?;
    let study_instance_uid = get_string(obj, dicom_dictionary_std::tags::STUDY_INSTANCE_UID)?;
    let series_instance_uid = get_string(obj, SERIES_INSTANCE_UID)?;
    let frame_of_reference_uid = get_string(obj, FRAME_OF_REFERENCE_UID)?;

    Ok(DicomFile::Image(Image {
        file_info,
        patient_id,
        patient_name,
        sop,
        study_instance_uid,
        series_instance_uid,
        frame_of_reference_uid,
    }))
}

/// Reads an RT Struct file and creates a DicomFile representing the RT Struct.
///
/// # Arguments
///
/// * `path` - A path to the RT Struct file.
/// * `obj` - A reference to a DefaultDicomObject that contains the metadata of the RT Struct file.
///
/// # Returns
///
/// A Result containing a DicomFile representing the RT Struct, or a DicomError if an error occurred.
fn read_rtstruct<P: AsRef<Path>>(
    path: P,
    obj: &DefaultDicomObject,
) -> Result<DicomFile, DicomError> {
    let file_info = last_modified_time(path)?;
    let sop = sop_class(obj)?;

    let patient_id = get_string(obj, PATIENT_ID)?;
    let study_instance_uid = get_string(obj, dicom_dictionary_std::tags::STUDY_INSTANCE_UID)?;
    let series_instance_uid = get_string(obj, SERIES_INSTANCE_UID)?;
    let label = get_opt_string(obj, dicom_dictionary_std::tags::STRUCTURE_SET_LABEL)?;

    let referenced_frame_of_references = get_sequence(
        obj.element_opt(REFERENCED_FRAME_OF_REFERENCE_SEQUENCE)?,
        referenced_frame_of_reference,
    )?;

    Ok(DicomFile::RTStruct(RTStruct {
        file_info,
        patient_id,
        sop,
        study_instance_uid,
        series_instance_uid,
        label,
        referenced_frame_of_references,
    }))
}

/// Reads an RT Plan DICOM file and creates a `DicomFile` object.
///
/// # Arguments
///
/// * `path` - The path to the RT Plan Dicom file.
/// * `obj` - The default Dicom object to extract information from.
///
/// # Returns
///
/// Returns a `Result` containing a `DicomFile::RTPlan(RTPlan)` if successful, or a `DicomError` if an error occurred.
fn read_rtplan<P: AsRef<Path>>(path: P, obj: &DefaultDicomObject) -> Result<DicomFile, DicomError> {
    let file_info = last_modified_time(path)?;
    let sop = sop_class(obj)?;

    let patient_id = get_string(obj, PATIENT_ID)?;
    let patient_name = get_string(obj, PATIENT_NAME)?;
    let plan_name = get_opt_string(obj, RT_PLAN_NAME)?;
    let plan_label = get_opt_string(obj, RT_PLAN_LABEL)?;
    let referenced_structure_sets = get_sequence(
        obj.element_opt(REFERENCED_STRUCTURE_SET_SEQUENCE)?,
        referenced_sop_class,
    )?;
    Ok(DicomFile::RTPlan(RTPlan {
        file_info,
        patient_id,
        patient_name,
        sop,
        plan_name,
        plan_label,
        referenced_structure_sets,
    }))
}

/// Reads an RT Dose DICOM file and creates a `DicomFile` object.
///
/// # Arguments
///
/// * `path` - The path to the RT Dose Dicom file.
/// * `obj` - The default Dicom object to extract information from.
///
/// # Returns
///
/// Returns a `Result` containing a `DicomFile::RTDose(RTDose)` if successful, or a `DicomError` if an error occurred.
fn read_rtdose<P: AsRef<Path>>(path: P, obj: &DefaultDicomObject) -> Result<DicomFile, DicomError> {
    let file_info = last_modified_time(path)?;
    let sop = sop_class(obj)?;

    let patient_id = get_string(obj, PATIENT_ID)?;
    let referenced_rtplan_sequence = get_sequence(
        obj.element_opt(REFERENCED_RT_PLAN_SEQUENCE)?,
        referenced_sop_class,
    )?;
    Ok(DicomFile::RTDose(RTDose {
        file_info,
        patient_id,
        sop,
        referenced_rtplan_sequence,
    }))
}

/// Returns the value of a DICOM element as a string.
///
/// # Arguments
///
/// * `obj` - A reference to an `InMemDicomObject` where the element can be found.
/// * `tag` - The tag of the DICOM element.
///
/// # Returns
///
/// A `Result` containing the value of the DICOM element as a string, or a `DicomError` if an error occurred.
fn get_string(obj: &InMemDicomObject, tag: Tag) -> Result<String, DicomError> {
    Ok(obj.element(tag)?.to_str()?.to_string())
}

/// Retrieves the value of an DICOM element as a string. If the element is not found, an empty string is returned.
///
/// # Arguments
///
/// * `obj` - A reference to the `InMemDicomObject` from which to retrieve the element.
/// * `tag` - The tag of the DICOM element to retrieve.
///
/// # Returns
///
/// * `Ok(String)` - If the element exists and has a valid string value, returns the string value.
/// * `Ok(<default>)` - If the element does not exist, returns the default value for the string type.
/// * `Err(DicomError)` - If there was an error retrieving the element or converting its value to a string.
fn get_opt_string(obj: &InMemDicomObject, tag: Tag) -> Result<String, DicomError> {
    match obj.element_opt(tag)? {
        None => Ok(Default::default()),
        Some(e) => Ok(e.to_str()?.to_string()),
    }
}

/// Retrieves the last modified time of the file at the given path.
///
/// # Arguments
///
/// * `path` - The path of the file.
///
/// # Returns
///
/// Returns a `Result` containing a `FileInfo` struct with the path of the file and the last modified time.
///
/// If successful, `Ok(FileInfo)` is returned, otherwise an `Err(DicomError)` is returned.
fn last_modified_time<P: AsRef<Path>>(path: P) -> Result<FileInfo, DicomError> {
    let meta = std::fs::metadata(path.as_ref())?;
    let time = meta.modified()?;
    Ok(FileInfo {
        path: path.as_ref().to_path_buf(),
        last_modified: Some(time),
    })
}

/// Reads the SOP Class UID and SOP Instance UID from the provided `InMemDicomObject`.
///
/// # Arguments
///
/// * `obj` - The `InMemDicomObject` to read from.
///
/// # Returns
///
/// Returns a `Result` containing a `SopClass` if successful, or a `DicomError` if an error occurred.
///
/// # Errors
///
/// Returns a `DicomError` if any of the following conditions are met:
///
/// * The SOP Class UID is not present in the `InMemDicomObject`.
/// * The SOP Class UID is not a valid string.
/// * The SOP Instance UID is not present in the `InMemDicomObject`.
/// * The SOP Instance UID is not a valid string.
/// * Any other error occurred while reading the `InMemDicomObject`.
///
fn sop_class(obj: &InMemDicomObject) -> Result<SopClass, DicomError> {
    let class_uid = get_opt_string(obj, SOP_CLASS_UID)?;
    let instance_uid = get_opt_string(obj, SOP_INSTANCE_UID)?;
    Ok(SopClass {
        class_uid,
        instance_uid,
    })
}

/// Extracts the referenced SOP class UID and referenced SOP instance UID from an InMemDicomObject.
///
/// # Arguments
///
/// * `obj` - A reference to the InMemDicomObject to extract the information from.
///
/// # Returns
///
/// A Result containing a ReferencedSopClass if the extraction is successful, or a DicomError otherwise.
fn referenced_sop_class(obj: &InMemDicomObject) -> Result<ReferencedSopClass, DicomError> {
    let ref_class_uid = get_opt_string(obj, REFERENCED_SOP_CLASS_UID)?;
    let ref_instance_uid = get_opt_string(obj, REFERENCED_SOP_INSTANCE_UID)?;
    Ok(ReferencedSopClass {
        ref_class_uid,
        ref_instance_uid,
    })
}

/// Returns the referenced frame of reference.
///
/// This function takes an `InMemDicomObject` as input, which represents a DICOM object in memory.
/// It returns a `Result` which contains the `ReferencedFrameOfReference` if successful, or a `DicomError` if an error occurs.
///
/// # Arguments
///
/// * `item` - An `InMemDicomObject` reference
///
/// # Returns
///
/// A `Result` containing the `ReferencedFrameOfReference` if successful, or a `DicomError` if an error occurs.
fn referenced_frame_of_reference(
    item: &InMemDicomObject,
) -> Result<ReferencedFrameOfReference, DicomError> {
    let frame_of_reference_uid = get_opt_string(item, FRAME_OF_REFERENCE_UID)?;
    let rt_referenced_study_sequence = get_sequence(
        item.element_opt(RT_REFERENCED_STUDY_SEQUENCE)?,
        rt_referenced_study,
    )?;
    Ok(ReferencedFrameOfReference {
        frame_of_reference_uid,
        rt_referenced_study_sequence,
    })
}

/// Constructs an `RTReferencedStudy` object from an `&InMemDicomObject`.
///
/// # Arguments
///
/// * `item` - A reference to an `InMemDicomObject` from which the `RTReferencedStudy` object will be created.
///
/// # Returns
///
/// Returns a `Result` containing the `RTReferencedStudy` object if successful, or a `DicomError` otherwise.
fn rt_referenced_study(item: &InMemDicomObject) -> Result<RTReferencedStudy, DicomError> {
    let referenced_sop = referenced_sop_class(item)?;
    let rt_referenced_series = get_sequence(
        item.element_opt(RT_REFERENCED_SERIES_SEQUENCE)?,
        rt_referenced_series,
    )?;
    Ok(RTReferencedStudy {
        referenced_sop,
        rt_referenced_series,
    })
}

/// Construct an `RTReferencedSerie` object from an `&InMemDicomObject`.
///
/// # Arguments
///
/// * `item` - The input `InMemDicomObject` to convert.
///
/// # Returns
///
///Returns `Ok(RTReferencedSerie)` if the conversion succeeds, or
/// `Err(DicomError)` if an error occurs during the conversion.
fn rt_referenced_series(item: &InMemDicomObject) -> Result<RTReferencedSerie, DicomError> {
    let instance_uid = get_opt_string(item, SERIES_INSTANCE_UID)?;
    let contour_image_sequence = get_sequence(
        item.element_opt(CONTOUR_IMAGE_SEQUENCE)?,
        referenced_sop_class,
    )?;
    Ok(RTReferencedSerie {
        instance_uid,
        contour_image_sequence,
    })
}

/// Given a sequence and a closure `read_item`, this function retrieves the items in the sequence and applies
/// the `read_item` closure to each item. The closure should take an `InMemDicomObject` as input and return
/// a `Result<R, DicomError>`, where `R` is the type of the desired result.
///
/// # Arguments
///
/// * `seq` - An optional reference to an `InMemElement`, which represents the sequence.
/// * `read_item` - A closure that takes an `InMemDicomObject` as input and returns a `Result<R, DicomError>`.
///
/// # Returns
///
/// * `Ok(Vec<R>)` - A vector containing the results of applying the `read_item` closure to each item in the sequence.
/// * `Err(DicomError)` - If there was an error retrieving the items or applying the `read_item` closure.
fn get_sequence<F, R>(seq: Option<&InMemElement>, read_item: F) -> Result<Vec<R>, DicomError>
where
    F: Fn(&InMemDicomObject) -> Result<R, DicomError>,
{
    if seq.is_none() {
        return Ok(vec![]);
    }
    let seq = seq.unwrap();
    if seq.vr() != VR::SQ {
        return Ok(vec![]);
    }
    let items = seq.items().unwrap();
    let mut v = vec![];
    for item in items {
        let ref_sop = read_item(item)?;
        v.push(ref_sop);
    }
    Ok(v)
}
