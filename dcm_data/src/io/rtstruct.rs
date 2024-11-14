use crate::io::common::read_sop;
use crate::io::{
    da_tm_to_ndt_opt, from_seq, from_seq_opt, to_date_opt, to_f64_opt, to_f64s, to_int, to_int_opt,
    to_ints_opt, to_string, to_string_opt, DcmIOError,
};
use crate::{
    ApprovalStatus, Contour, ContourGeometry, PersonName, RTReferencedSerie, RTReferencedStudy,
    RTRoiObservation, RTStruct, ReferencedFrameOfReference, RoiContour, Sop, StructureSetROI,
};
use dicom_dictionary_std::tags::{
    ACCESSION_NUMBER, APPROVAL_STATUS, CONTOUR_DATA, CONTOUR_GEOMETRIC_TYPE,
    CONTOUR_IMAGE_SEQUENCE, CONTOUR_NUMBER, CONTOUR_SEQUENCE, DEIDENTIFICATION_METHOD,
    FRAME_OF_REFERENCE_UID, INSTANCE_CREATION_DATE, INSTANCE_CREATION_TIME, INSTANCE_NUMBER,
    MANUFACTURER, MANUFACTURER_MODEL_NAME, NUMBER_OF_CONTOUR_POINTS, OBSERVATION_NUMBER,
    PATIENT_BIRTH_DATE, PATIENT_ID, PATIENT_IDENTITY_REMOVED, PATIENT_NAME, PATIENT_SEX,
    POSITION_REFERENCE_INDICATOR, REFERENCED_FRAME_OF_REFERENCE_SEQUENCE,
    REFERENCED_FRAME_OF_REFERENCE_UID, REFERENCED_ROI_NUMBER, REFERENCED_SOP_CLASS_UID,
    REFERENCED_SOP_INSTANCE_UID, REFERRING_PHYSICIAN_NAME, ROI_CONTOUR_SEQUENCE, ROI_DISPLAY_COLOR,
    ROI_GENERATION_ALGORITHM, ROI_GENERATION_DESCRIPTION, ROI_INTERPRETER, ROI_NAME, ROI_NUMBER,
    ROI_VOLUME, RTROI_INTERPRETED_TYPE, RTROI_OBSERVATIONS_SEQUENCE, RT_REFERENCED_SERIES_SEQUENCE,
    RT_REFERENCED_STUDY_SEQUENCE, SERIES_INSTANCE_UID, SERIES_NUMBER, SOFTWARE_VERSIONS,
    SOP_CLASS_UID, SOP_INSTANCE_UID, SPECIFIC_CHARACTER_SET, STRUCTURE_SET_DATE,
    STRUCTURE_SET_LABEL, STRUCTURE_SET_ROI_SEQUENCE, STRUCTURE_SET_TIME, STUDY_DATE, STUDY_ID,
    STUDY_INSTANCE_UID, STUDY_TIME,
};
use dicom_dictionary_std::uids::RT_STRUCTURE_SET_STORAGE;
use dicom_object::InMemDicomObject;
use std::path::Path;
use std::str::FromStr;

pub fn read_rtstruct<P: AsRef<Path>>(path: P) -> Result<RTStruct, DcmIOError> {
    let file_obj = dicom_object::open_file(path.as_ref())?;
    let obj = file_obj.into_inner();
    let sop_class_uid = to_string(&obj, SOP_CLASS_UID)?;
    if sop_class_uid != RT_STRUCTURE_SET_STORAGE {
        return Err(DcmIOError::NoMatchingSopClassUID(sop_class_uid));
    }
    Ok(RTStruct {
        specific_character_set: to_string(&obj, SPECIFIC_CHARACTER_SET)?,
        instance_creation_dt: da_tm_to_ndt_opt(
            &obj,
            INSTANCE_CREATION_DATE,
            INSTANCE_CREATION_TIME,
        )?,
        sop: read_sop(&obj, SOP_CLASS_UID, SOP_INSTANCE_UID)?,
        study_dt: da_tm_to_ndt_opt(&obj, STUDY_DATE, STUDY_TIME)?,
        accession_number: to_string_opt(&obj, ACCESSION_NUMBER)?,
        manufacturer: to_string_opt(&obj, MANUFACTURER)?,
        referring_physician_name: to_string_opt(&obj, REFERRING_PHYSICIAN_NAME)?
            .map(|s| PersonName::from_str(&s).unwrap()),
        manufacturer_model_name: to_string_opt(&obj, MANUFACTURER_MODEL_NAME)?,
        patient_name: PersonName::from_str(&to_string(&obj, PATIENT_NAME)?).unwrap(),
        patient_id: to_string(&obj, PATIENT_ID)?,
        patient_birth_date: to_date_opt(&obj, PATIENT_BIRTH_DATE)?,
        patient_sex: to_string(&obj, PATIENT_SEX)?,
        patient_identity_removed: to_string(&obj, PATIENT_IDENTITY_REMOVED)? == "YES",
        deidentification_method: to_string_opt(&obj, DEIDENTIFICATION_METHOD)?,
        software_versions: to_string_opt(&obj, SOFTWARE_VERSIONS)?,
        study_instance_uid: to_string(&obj, STUDY_INSTANCE_UID)?,
        series_instance_uid: to_string(&obj, SERIES_INSTANCE_UID)?,
        study_id: to_string_opt(&obj, STUDY_ID)?,
        series_number: to_int(&obj, SERIES_NUMBER)?,
        instance_number: to_int(&obj, INSTANCE_NUMBER)?,
        frame_of_reference_uid: to_string(&obj, FRAME_OF_REFERENCE_UID)?,
        position_reference_indicator: to_string_opt(&obj, POSITION_REFERENCE_INDICATOR)?,
        structure_set_label: to_string(&obj, STRUCTURE_SET_LABEL)?,
        structure_set_dt: da_tm_to_ndt_opt(&obj, STRUCTURE_SET_DATE, STRUCTURE_SET_TIME)?,
        referenced_frame_of_reference_seq: from_seq(
            &obj,
            REFERENCED_FRAME_OF_REFERENCE_SEQUENCE,
            referenced_frame_of_reference,
        )?,
        structure_set_roi_sequence: from_seq(&obj, STRUCTURE_SET_ROI_SEQUENCE, structure_set_roi)?,
        roi_contour_sequence: from_seq(&obj, ROI_CONTOUR_SEQUENCE, roi_contour)?,
        rt_roi_observations_sequence: from_seq(
            &obj,
            RTROI_OBSERVATIONS_SEQUENCE,
            rt_roi_observation,
        )?,
        approval_status: to_string_opt(&obj, APPROVAL_STATUS)?
            .map(|s| ApprovalStatus::from_str(&s).unwrap()),
    })
}

fn referenced_frame_of_reference(
    item: &InMemDicomObject,
) -> Result<ReferencedFrameOfReference, DcmIOError> {
    Ok(ReferencedFrameOfReference {
        frame_of_reference_uid: to_string(item, FRAME_OF_REFERENCE_UID)?,
        rt_referenced_study_sequence: from_seq(
            item,
            RT_REFERENCED_STUDY_SEQUENCE,
            rt_referenced_study,
        )?,
    })
}

fn rt_referenced_study(item: &InMemDicomObject) -> Result<RTReferencedStudy, DcmIOError> {
    Ok(RTReferencedStudy {
        referenced_sop: read_sop(item, REFERENCED_SOP_CLASS_UID, REFERENCED_SOP_INSTANCE_UID)?,
        rt_referenced_series_sequence: from_seq(
            item,
            RT_REFERENCED_SERIES_SEQUENCE,
            rt_referenced_series,
        )?,
    })
}

fn rt_referenced_series(item: &InMemDicomObject) -> Result<RTReferencedSerie, DcmIOError> {
    Ok(RTReferencedSerie {
        series_instance_uid: to_string(item, SERIES_INSTANCE_UID)?,
        contour_image_sequence: from_seq(item, CONTOUR_IMAGE_SEQUENCE, contour_image)?,
    })
}
fn contour_image(item: &InMemDicomObject) -> Result<Sop, DcmIOError> {
    read_sop(item, REFERENCED_SOP_CLASS_UID, REFERENCED_SOP_INSTANCE_UID)
}
fn structure_set_roi(item: &InMemDicomObject) -> Result<StructureSetROI, DcmIOError> {
    Ok(StructureSetROI {
        roi_number: to_int(item, ROI_NUMBER)?,
        referenced_frame_of_reference_uid: to_string(item, REFERENCED_FRAME_OF_REFERENCE_UID)?,
        roi_name: to_string_opt(item, ROI_NAME)?,
        roi_generation_algorithm: to_string_opt(item, ROI_GENERATION_ALGORITHM)?,
        roi_generation_description: to_string_opt(item, ROI_GENERATION_DESCRIPTION)?,
        roi_volume: to_f64_opt(item, ROI_VOLUME)?,
    })
}
fn roi_contour(item: &InMemDicomObject) -> Result<RoiContour, DcmIOError> {
    let colors: Vec<u8> = to_ints_opt(item, ROI_DISPLAY_COLOR)?.unwrap();
    let n = colors.len();
    let display_color = if n == 3 {
        Ok(Some([colors[0], colors[1], colors[2]]))
    } else if n == 0 {
        Ok(None)
    } else {
        Err(DcmIOError::InvalidRGBString(
            to_string_opt(item, ROI_DISPLAY_COLOR)?.unwrap_or_default(),
        ))
    }?;
    Ok(RoiContour {
        roi_display_color: display_color,
        contour_sequence: from_seq_opt(item, CONTOUR_SEQUENCE, contour)?,
        referenced_roi_number: to_int(item, REFERENCED_ROI_NUMBER)?,
    })
}
fn contour(item: &InMemDicomObject) -> Result<Contour, DcmIOError> {
    Ok(Contour {
        contour_number: to_int_opt(item, CONTOUR_NUMBER)?,
        contour_image_sequence: from_seq_opt(item, CONTOUR_IMAGE_SEQUENCE, contour_image)?,
        contour_geometry_type: ContourGeometry::from_str(&to_string(
            item,
            CONTOUR_GEOMETRIC_TYPE,
        )?)?,
        number_of_contour_points: to_int(item, NUMBER_OF_CONTOUR_POINTS)?,
        contour_data: to_f64s(item, CONTOUR_DATA)?,
    })
}
fn rt_roi_observation(item: &InMemDicomObject) -> Result<RTRoiObservation, DcmIOError> {
    Ok(RTRoiObservation {
        observation_number: to_int(item, OBSERVATION_NUMBER)?,
        referenced_roi_number: to_int(item, REFERENCED_ROI_NUMBER)?,
        rt_roi_interpreted_type: to_string_opt(item, RTROI_INTERPRETED_TYPE)?,
        roi_interpreter: to_string_opt(item, ROI_INTERPRETER)?,
    })
}
