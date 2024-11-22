use crate::io::common::{code_item, read_sop};
use crate::io::{
    da_tm_to_ndt_opt, from_seq, from_seq_opt, to_date_opt, to_f64, to_f64_opt, to_f64s, to_int,
    to_int_opt, to_string, to_string_opt, DcmIOError,
};
use crate::{
    DoseSummationType, DoseType, DoseUnit, PersonName, PhotometricInterpretation,
    PixelRepresentation, PlanOverview, PrescriptionOverview, RTDose, ReferencedControlPoint,
    ReferencedFractionGroup, ReferencedFractionGroupReferencedBeam, ReferencedRTPlan,
    ReferencedTreatmentRecord, ReferencedTreatmentRecordReferencedBeam, Sop,
    TissueHeterogeneityCorrection,
};
use dicom_dictionary_std::tags::{
    ACCESSION_NUMBER, BITS_ALLOCATED, BITS_STORED, COLUMNS, CURRENT_FRACTION_NUMBER,
    DEIDENTIFICATION_METHOD, DOSE_COMMENT, DOSE_GRID_SCALING, DOSE_SUMMATION_TYPE, DOSE_TYPE,
    DOSE_UNITS, ENTITY_LONG_LABEL, FRAME_INCREMENT_POINTER, FRAME_OF_REFERENCE_UID,
    GRID_FRAME_OFFSET_VECTOR, HIGH_BIT, IMAGE_ORIENTATION_PATIENT, IMAGE_POSITION_PATIENT,
    INSTANCE_CREATION_DATE, INSTANCE_CREATION_TIME, INSTANCE_NUMBER, MANUFACTURER,
    MANUFACTURER_MODEL_NAME, NUMBER_OF_FRACTIONS_INCLUDED, NUMBER_OF_FRAMES, PATIENT_BIRTH_DATE,
    PATIENT_ID, PATIENT_IDENTITY_REMOVED, PATIENT_NAME, PATIENT_SEX, PHOTOMETRIC_INTERPRETATION,
    PIXEL_DATA, PIXEL_REPRESENTATION, PIXEL_SPACING, PLAN_OVERVIEW_INDEX, PLAN_OVERVIEW_SEQUENCE,
    POSITION_REFERENCE_INDICATOR, PRESCRIPTION_OVERVIEW_SEQUENCE, REFERENCED_BEAM_NUMBER,
    REFERENCED_BEAM_SEQUENCE, REFERENCED_CONTROL_POINT_SEQUENCE, REFERENCED_FRACTION_GROUP_NUMBER,
    REFERENCED_FRACTION_GROUP_SEQUENCE, REFERENCED_IMAGE_SEQUENCE, REFERENCED_PLAN_OVERVIEW_INDEX,
    REFERENCED_ROI_NUMBER, REFERENCED_RT_PLAN_SEQUENCE, REFERENCED_SOP_CLASS_UID,
    REFERENCED_SOP_INSTANCE_UID, REFERENCED_START_CONTROL_POINT_INDEX,
    REFERENCED_STOP_CONTROL_POINT_INDEX, REFERENCED_STRUCTURE_SET_SEQUENCE,
    REFERENCED_TREATMENT_RECORD_SEQUENCE, REFERRING_PHYSICIAN_NAME, ROWS, RT_PLAN_LABEL,
    SAMPLES_PER_PIXEL, SERIES_INSTANCE_UID, SERIES_NUMBER, SLICE_THICKNESS, SOFTWARE_VERSIONS,
    SOP_CLASS_UID, SOP_INSTANCE_UID, SPECIFIC_CHARACTER_SET, STUDY_DATE, STUDY_ID,
    STUDY_INSTANCE_UID, STUDY_TIME, TISSUE_HETEROGENEITY_CORRECTION, TOTAL_PRESCRIPTION_DOSE,
    TREATMENT_SITE, TREATMENT_SITE_CODE_SEQUENCE,
};
use dicom_dictionary_std::uids::RT_DOSE_STORAGE;
use dicom_object::{DefaultDicomObject, InMemDicomObject};
use dicom_pixeldata::PixelDecoder;
use std::path::Path;
use std::str::FromStr;

/// Reads an RTDose DICOM file from the specified path and parses it into an `RTDose` structure.
///
/// This function opens a DICOM file, decodes its pixel data, and extracts relevant DICOM attributes
/// to populate the `RTDose` structure. It performs validation on various fields to ensure they
/// conform to expected formats.
///
/// # Arguments
///
/// * `path` - A reference to a path that points to the RTDose DICOM file.
///
/// # Returns
///
/// * `Result<RTDose, DcmIOError>` - Returns an `RTDose` structure if the file is successfully
///   read and parsed, otherwise returns a `DcmIOError`.
///
/// # Errors
///
/// This function will return an error in the following cases:
///
/// * If the file cannot be opened or read.
/// * If the pixel data cannot be decoded.
/// * If the `SOPClassUID` does not match `RT_DOSE_STORAGE`.
/// * If any of the expected DICOM attributes have invalid or unexpected values.
///
/// # Example
///
/// ```rust
/// use std::path::Path;
/// use dcm_data::io::read_rtdose;
/// let result = read_rtdose("tests/resources/RD1.2.752.243.1.1.20220722130644614.2020.66722.dcm");
/// match result {
///     Ok(rtdose) => println!("Successfully read RTDose file."),
///     Err(e) => eprintln!("Error reading RTDose file: {:?}", e),
/// }
/// ```
pub fn read_rtdose<P: AsRef<Path>>(path: P) -> Result<RTDose, DcmIOError> {
    let file_obj = dicom_object::open_file(path.as_ref())?;
    obj_to_rtdose(file_obj)
}

/// Converts a DICOM object to an `RTDose` structure.
///
/// This function takes a `DefaultDicomObject`, decodes its pixel data, and extracts
/// necessary DICOM attributes to create and return an `RTDose` structure. It performs
/// various checks and validations to ensure the attributes conform to expected formats.
///
/// # Arguments
///
/// * `obj` - The `DefaultDicomObject` that represents the DICOM object to be converted.
///
/// # Returns
///
/// * `Result<RTDose, DcmIOError>` - Returns an `RTDose` structure if successful, otherwise returns an error.
///
/// # Errors
///
/// This function will return an error in the following cases:
///
/// * If the pixel data cannot be decoded.
/// * If the `SOPClassUID` does not match `RT_DOSE_STORAGE`.
/// * If any of the expected DICOM attributes have invalid or unexpected values.
///
/// # Example
///
/// ```rust
/// use dicom_object::DefaultDicomObject;
/// use std::path::Path;
/// use dicom_object::open_file;
/// use dcm_data::io::obj_to_rtdose;
///
/// let path = Path::new("tests/resources/RD1.2.752.243.1.1.20220722130644614.2020.66722.dcm");
/// let file_obj = open_file(path).unwrap();
/// let rtdose = obj_to_rtdose(file_obj);
/// match rtdose {
///     Ok(rtdose) => println!("Successfully converted DICOM object to RTDose."),
///     Err(e) => eprintln!("Error converting DICOM object to RTDose: {:?}", e),
/// }
/// ```
pub fn obj_to_rtdose(obj: DefaultDicomObject) -> Result<RTDose, DcmIOError> {
    let decoded_pixels = obj.decode_pixel_data()?;
    let pixel_data = decoded_pixels.to_ndarray::<f64>()?;
    let obj = obj.into_inner();
    let sop_class_uid = to_string(&obj, SOP_CLASS_UID)?;
    if sop_class_uid != RT_DOSE_STORAGE {
        return Err(DcmIOError::NoMatchingSopClassUID(sop_class_uid));
    }

    let ipp = to_f64s(&obj, IMAGE_POSITION_PATIENT)?;
    if ipp.len() != 3 {
        return Err(DcmIOError::InvalidImagePositionPatient(ipp));
    }
    let iop = to_f64s(&obj, IMAGE_ORIENTATION_PATIENT)?;
    if iop.len() != 6 {
        return Err(DcmIOError::InvalidImageOrientationPatient(iop));
    }
    let pixel_spacing = to_f64s(&obj, PIXEL_SPACING)?;
    if pixel_spacing.len() != 2 {
        return Err(DcmIOError::InvalidPixelSpacing(pixel_spacing));
    }

    Ok(RTDose {
        specific_character_set: to_string(&obj, SPECIFIC_CHARACTER_SET)?,
        instance_creation_dt: da_tm_to_ndt_opt(
            &obj,
            INSTANCE_CREATION_DATE,
            INSTANCE_CREATION_TIME,
        )?,
        image_type: vec![],
        sop: read_sop(&obj, SOP_CLASS_UID, SOP_INSTANCE_UID)?,
        study_dt: da_tm_to_ndt_opt(&obj, STUDY_DATE, STUDY_TIME)?,
        content_dt: None,
        accession_number: to_string_opt(&obj, ACCESSION_NUMBER)?,
        ref_physician_name: None,
        station_name: None,
        manufacturer: to_string_opt(&obj, MANUFACTURER)?,
        referring_physician_name: to_string_opt(&obj, REFERRING_PHYSICIAN_NAME)?
            .map(|s| PersonName::from_str(&s).unwrap()),
        manufacturer_model_name: to_string_opt(&obj, MANUFACTURER_MODEL_NAME)?,
        irradiation_event_uid: "".to_string(),
        patient_name: PersonName::from_str(&to_string(&obj, PATIENT_NAME)?).unwrap(),
        patient_id: to_string(&obj, PATIENT_ID)?,
        patient_birth_date: to_date_opt(&obj, PATIENT_BIRTH_DATE)?,
        patient_sex: to_string(&obj, PATIENT_SEX)?,
        patient_identity_removed: to_string(&obj, PATIENT_IDENTITY_REMOVED)? == "YES",
        deidentification_method: to_string_opt(&obj, DEIDENTIFICATION_METHOD)?,
        slice_thickness: to_f64_opt(&obj, SLICE_THICKNESS)?,
        software_versions: to_string_opt(&obj, SOFTWARE_VERSIONS)?,
        study_instance_uid: to_string(&obj, STUDY_INSTANCE_UID)?,
        series_instance_uid: to_string(&obj, SERIES_INSTANCE_UID)?,
        study_id: to_string_opt(&obj, STUDY_ID)?,
        series_number: to_int(&obj, SERIES_NUMBER)?,
        instance_number: to_int(&obj, INSTANCE_NUMBER)?,
        image_position_patient: [ipp[0], ipp[1], ipp[2]],
        image_orientation_patient: [iop[0], iop[1], iop[2], iop[3], iop[4], iop[5]],
        frame_of_reference_uid: to_string(&obj, FRAME_OF_REFERENCE_UID)?,
        position_reference_indicator: to_string_opt(&obj, POSITION_REFERENCE_INDICATOR)?,
        samples_per_pixel: to_int(&obj, SAMPLES_PER_PIXEL)?,
        photometric_interpretation: PhotometricInterpretation::from_str(&to_string(
            &obj,
            PHOTOMETRIC_INTERPRETATION,
        )?)?,
        number_of_frames: to_int(&obj, NUMBER_OF_FRAMES)?,
        frame_increment_pointer: to_string(&obj, FRAME_INCREMENT_POINTER)?,
        rows: to_int(&obj, ROWS)?,
        columns: to_int(&obj, COLUMNS)?,
        pixel_spacing: [pixel_spacing[0], pixel_spacing[1]],
        bits_allocated: to_int(&obj, BITS_ALLOCATED)?,
        bits_stored: to_int(&obj, BITS_STORED)?,
        high_bit: to_int(&obj, HIGH_BIT)?,
        pixel_representation: PixelRepresentation::from_str(&to_string(
            &obj,
            PIXEL_REPRESENTATION,
        )?)?,
        dose_units: DoseUnit::from_str(&to_string(&obj, DOSE_UNITS)?).unwrap(),
        dose_type: DoseType::from_str(&to_string(&obj, DOSE_TYPE)?).unwrap(),
        dose_comment: to_string_opt(&obj, DOSE_COMMENT)?,
        dose_summation_type: DoseSummationType::from_str(&to_string(&obj, DOSE_SUMMATION_TYPE)?)
            .unwrap(),
        grid_frame_offset_vector: to_f64s(&obj, GRID_FRAME_OFFSET_VECTOR)?,
        dose_grid_scaling: to_f64(&obj, DOSE_GRID_SCALING)?,
        tissue_heterogeneity_correction: to_string_opt(&obj, TISSUE_HETEROGENEITY_CORRECTION)?
            .map(|s| TissueHeterogeneityCorrection::from_str(&s).unwrap()),
        referenced_treatment_record_sequence: from_seq_opt(
            &obj,
            REFERENCED_TREATMENT_RECORD_SEQUENCE,
            referenced_treatment_record,
        )?,
        referenced_rt_plan_sequence: from_seq_opt(
            &obj,
            REFERENCED_RT_PLAN_SEQUENCE,
            referenced_rt_plan,
        )?,
        plan_overview_sequence: from_seq_opt(&obj, PLAN_OVERVIEW_SEQUENCE, plan_overview)?,
        pixel_data_bytes: obj.element(PIXEL_DATA)?.to_bytes()?.to_vec(),
        pixel_data,
    })
}

fn referenced_treatment_record(
    item: &InMemDicomObject,
) -> Result<ReferencedTreatmentRecord, DcmIOError> {
    Ok(ReferencedTreatmentRecord {
        referenced_sop: read_sop(item, REFERENCED_SOP_CLASS_UID, REFERENCED_SOP_INSTANCE_UID)?,
        referenced_beam_sequence: from_seq(item, REFERENCED_BEAM_SEQUENCE, referenced_beam)?,
    })
}

fn referenced_beam(
    item: &InMemDicomObject,
) -> Result<ReferencedTreatmentRecordReferencedBeam, DcmIOError> {
    Ok(ReferencedTreatmentRecordReferencedBeam {
        referenced_beam_number: to_int(item, REFERENCED_BEAM_NUMBER)?,
    })
}

fn referenced_rt_plan(item: &InMemDicomObject) -> Result<ReferencedRTPlan, DcmIOError> {
    Ok(ReferencedRTPlan {
        referenced_sop: read_sop(item, REFERENCED_SOP_CLASS_UID, REFERENCED_SOP_INSTANCE_UID)?,
        referenced_fraction_group: from_seq_opt(
            item,
            REFERENCED_FRACTION_GROUP_SEQUENCE,
            referenced_fraction_group,
        )?,
        referenced_plan_overview_index: to_int_opt(item, REFERENCED_PLAN_OVERVIEW_INDEX)?,
    })
}

fn referenced_fraction_group(
    item: &InMemDicomObject,
) -> Result<ReferencedFractionGroup, DcmIOError> {
    Ok(ReferencedFractionGroup {
        referenced_beam_sequence: from_seq(
            item,
            REFERENCED_BEAM_SEQUENCE,
            referenced_fraction_group_referenced_beam,
        )?,
        referenced_brachy_application_setup_sequence: vec![],
        referenced_fraction_group_number: to_int(item, REFERENCED_FRACTION_GROUP_NUMBER)?,
    })
}

fn referenced_fraction_group_referenced_beam(
    item: &InMemDicomObject,
) -> Result<ReferencedFractionGroupReferencedBeam, DcmIOError> {
    Ok(ReferencedFractionGroupReferencedBeam {
        referenced_beam_number: to_int(item, REFERENCED_BEAM_NUMBER)?,
        referenced_control_point_sequence: from_seq(
            item,
            REFERENCED_CONTROL_POINT_SEQUENCE,
            referenced_control_point,
        )?,
    })
}

fn referenced_control_point(item: &InMemDicomObject) -> Result<ReferencedControlPoint, DcmIOError> {
    Ok(ReferencedControlPoint {
        start: to_int(item, REFERENCED_START_CONTROL_POINT_INDEX)?,
        end: to_int(item, REFERENCED_STOP_CONTROL_POINT_INDEX)?,
    })
}

fn plan_overview(item: &InMemDicomObject) -> Result<PlanOverview, DcmIOError> {
    Ok(PlanOverview {
        referenced_image_sequence: from_seq_opt(
            item,
            REFERENCED_IMAGE_SEQUENCE,
            |item| -> Result<Sop, DcmIOError> {
                read_sop(item, REFERENCED_SOP_CLASS_UID, REFERENCED_SOP_INSTANCE_UID)
            },
        )?,
        current_fraction_number: to_int(item, CURRENT_FRACTION_NUMBER)?,
        rt_plan_label: to_string_opt(item, RT_PLAN_LABEL)?,
        referenced_structure_set_sequence: from_seq_opt(
            item,
            REFERENCED_STRUCTURE_SET_SEQUENCE,
            |item| -> Result<Sop, DcmIOError> {
                read_sop(item, REFERENCED_SOP_CLASS_UID, REFERENCED_SOP_INSTANCE_UID)
            },
        )?,
        prescription_overview_sequence: from_seq_opt(
            item,
            PRESCRIPTION_OVERVIEW_SEQUENCE,
            prescription_overview,
        )?,
        plan_overview_index: to_int(item, PLAN_OVERVIEW_INDEX)?,
        number_of_fractions_included: to_int(item, NUMBER_OF_FRACTIONS_INCLUDED)?,
        treatment_site: to_string_opt(item, TREATMENT_SITE)?,
        treatment_site_code_sequence: from_seq_opt(item, TREATMENT_SITE_CODE_SEQUENCE, code_item)?,
    })
}

fn prescription_overview(item: &InMemDicomObject) -> Result<PrescriptionOverview, DcmIOError> {
    Ok(PrescriptionOverview {
        referenced_roi_number: to_int(item, REFERENCED_ROI_NUMBER)?,
        total_prescription_dose: to_f64(item, TOTAL_PRESCRIPTION_DOSE)?,
        entity_long_label: to_string_opt(item, ENTITY_LONG_LABEL)?,
    })
}
