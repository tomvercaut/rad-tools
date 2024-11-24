use crate::io::common::{read_sop, referenced_sop};
use crate::io::{
    da_tm_to_ndt_opt, from_seq, from_seq_opt, to_date_opt, to_f64, to_f64_opt, to_f64s,
    to_f64s_opt, to_int, to_int_opt, to_rotation_direction_opt, to_string, to_string_opt,
    DcmIOError,
};
use crate::{
    ApprovalStatus, Beam, BeamDoseType, BeamLimitingDevice, BeamLimitingDevicePosition, BeamType,
    ControlPoint, FluenceMode, FractionGroup, PatientPosition, PatientSetup, PersonName,
    PrimaryDosimeterUnit, PrimaryFluenceMode, RTBeamLimitingDeviceType, RTPlan, RadiationType,
    ReferencedBeam, ReferencedBolus, ReferencedBrachyApplicationSetup, ReferencedDoseReference,
    Sop, TreatmentDeliveryType,
};
use dicom_dictionary_std::tags::{
    ACCESSION_NUMBER, ACCESSORY_CODE, APPROVAL_STATUS, BEAM_DESCRIPTION, BEAM_DOSE,
    BEAM_DOSE_MEANING, BEAM_DOSE_POINT_DEPTH, BEAM_DOSE_POINT_EQUIVALENT_DEPTH,
    BEAM_DOSE_POINT_SSD, BEAM_DOSE_TYPE, BEAM_LIMITING_DEVICE_ANGLE,
    BEAM_LIMITING_DEVICE_POSITION_SEQUENCE, BEAM_LIMITING_DEVICE_ROTATION_DIRECTION,
    BEAM_LIMITING_DEVICE_SEQUENCE, BEAM_METERSET, BEAM_NAME, BEAM_NUMBER, BEAM_SEQUENCE, BEAM_TYPE,
    BOLUS_DESCRIPTION, BOLUS_ID, BRACHY_APPLICATION_SETUP_DOSE,
    BRACHY_APPLICATION_SETUP_DOSE_SPECIFICATION_POINT, CONSTRAINT_WEIGHT, CONTROL_POINT_INDEX,
    CONTROL_POINT_SEQUENCE, CUMULATIVE_METERSET_WEIGHT, DEIDENTIFICATION_METHOD,
    DELIVERY_MAXIMUM_DOSE, DELIVERY_WARNING_DOSE, FINAL_CUMULATIVE_METERSET_WEIGHT, FLUENCE_MODE,
    FLUENCE_MODE_ID, FRACTION_GROUP_DESCRIPTION, FRACTION_GROUP_NUMBER, FRACTION_GROUP_SEQUENCE,
    FRACTION_PATTERN, FRAME_OF_REFERENCE_UID, GANTRY_ANGLE, GANTRY_PITCH_ANGLE,
    GANTRY_PITCH_ROTATION_DIRECTION, GANTRY_ROTATION_DIRECTION, INSTANCE_CREATION_DATE,
    INSTANCE_CREATION_TIME, ISOCENTER_POSITION, LEAF_JAW_POSITIONS, LEAF_POSITION_BOUNDARIES,
    MANUFACTURER, MANUFACTURER_MODEL_NAME, NOMINAL_BEAM_ENERGY, NUMBER_OF_BEAMS, NUMBER_OF_BLOCKS,
    NUMBER_OF_BOLI, NUMBER_OF_BRACHY_APPLICATION_SETUPS, NUMBER_OF_COMPENSATORS,
    NUMBER_OF_CONTROL_POINTS, NUMBER_OF_FRACTIONS_PLANNED,
    NUMBER_OF_FRACTION_PATTERN_DIGITS_PER_DAY, NUMBER_OF_LEAF_JAW_PAIRS, NUMBER_OF_WEDGES,
    ORGAN_AT_RISK_FULL_VOLUME_DOSE, ORGAN_AT_RISK_LIMIT_DOSE, ORGAN_AT_RISK_MAXIMUM_DOSE,
    ORGAN_AT_RISK_OVERDOSE_VOLUME_FRACTION, PATIENT_BIRTH_DATE, PATIENT_ID,
    PATIENT_IDENTITY_REMOVED, PATIENT_NAME, PATIENT_POSITION, PATIENT_SETUP_NUMBER,
    PATIENT_SETUP_SEQUENCE, PATIENT_SEX, PATIENT_SUPPORT_ANGLE, PATIENT_SUPPORT_ROTATION_DIRECTION,
    PLAN_INTENT, POSITION_REFERENCE_INDICATOR, PRIMARY_DOSIMETER_UNIT,
    PRIMARY_FLUENCE_MODE_SEQUENCE, RADIATION_TYPE, REFERENCED_BEAM_NUMBER,
    REFERENCED_BEAM_SEQUENCE, REFERENCED_BOLUS_SEQUENCE,
    REFERENCED_BRACHY_APPLICATION_SETUP_NUMBER, REFERENCED_BRACHY_APPLICATION_SETUP_SEQUENCE,
    REFERENCED_DOSE_REFERENCE_NUMBER, REFERENCED_DOSE_REFERENCE_SEQUENCE,
    REFERENCED_DOSE_REFERENCE_UID, REFERENCED_DOSE_SEQUENCE, REFERENCED_PATIENT_SETUP_NUMBER,
    REFERENCED_ROI_NUMBER, REFERENCED_SOP_CLASS_UID, REFERENCED_SOP_INSTANCE_UID,
    REFERENCED_STRUCTURE_SET_SEQUENCE, REFERRING_PHYSICIAN_NAME, REPEAT_FRACTION_CYCLE_LENGTH,
    REVIEWER_NAME, REVIEW_DATE, REVIEW_TIME, RT_BEAM_LIMITING_DEVICE_TYPE, RT_PLAN_DATE,
    RT_PLAN_DESCRIPTION, RT_PLAN_GEOMETRY, RT_PLAN_LABEL, RT_PLAN_NAME, RT_PLAN_TIME,
    SERIES_INSTANCE_UID, SERIES_NUMBER, SOFTWARE_VERSIONS, SOP_CLASS_UID, SOP_INSTANCE_UID,
    SOURCE_AXIS_DISTANCE, SOURCE_TO_BEAM_LIMITING_DEVICE_DISTANCE, SOURCE_TO_SURFACE_DISTANCE,
    SPECIFIC_CHARACTER_SET, STUDY_DATE, STUDY_ID, STUDY_INSTANCE_UID, STUDY_TIME,
    TABLE_TOP_ECCENTRIC_ANGLE, TABLE_TOP_ECCENTRIC_ROTATION_DIRECTION, TABLE_TOP_LATERAL_POSITION,
    TABLE_TOP_LONGITUDINAL_POSITION, TABLE_TOP_PITCH_ANGLE, TABLE_TOP_PITCH_ROTATION_DIRECTION,
    TABLE_TOP_ROLL_ANGLE, TABLE_TOP_ROLL_ROTATION_DIRECTION, TABLE_TOP_VERTICAL_POSITION,
    TARGET_MINIMUM_DOSE, TARGET_PRESCRIPTION_DOSE, TARGET_UNDERDOSE_VOLUME_FRACTION,
    TREATMENT_DELIVERY_TYPE, TREATMENT_MACHINE_NAME, TREATMENT_PROTOCOLS,
};
use dicom_dictionary_std::uids::RT_PLAN_STORAGE;
use dicom_object::{DefaultDicomObject, InMemDicomObject};
use std::path::Path;
use std::str::FromStr;

/// Reads an RT Plan from a file at the given path and returns an `RTPlan` object.
///
/// # Arguments
///
/// * `path` - A reference to a type that implements the `AsRef<Path>` trait, representing the path to the RT Plan file.
///
/// # Returns
///
/// A `Result` which is:
///
/// * `Ok(RTPlan)` with the parsed RT Plan if the operation is successful.
/// * `Err(DcmIOError)` if an error occurs during reading or parsing the file.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The file cannot be opened or read.
/// * The file content cannot be parsed as a valid RT Plan.
/// * The SOP Class UID does not match the expected RT Plan storage UID.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use rad_tools_dcm_data::io::read_rtplan;
///
/// let rtplan = read_rtplan("tests/resources/RP1.2.752.243.1.1.20220722130644612.2000.30831.dcm");
/// match rtplan {
///     Ok(plan) => println!("Successfully read RT Plan: {:?}", plan),
///     Err(e) => eprintln!("Failed to read RT Plan: {}", e),
/// }
/// ```
///
/// # Dependencies
///
/// This function internally relies on the `dicom_object` crate to handle DICOM files.
///
/// # See Also
///
/// * `obj_to_rtplan` - Converts a DICOM object to an `RTPlan`.
pub fn read_rtplan<P: AsRef<Path>>(path: P) -> Result<RTPlan, DcmIOError> {
    let file_obj = dicom_object::open_file(path.as_ref())?;
    obj_to_rtplan(file_obj)
}

///
/// Converts a DICOM object to an `RTPlan`.
///
/// # Arguments
///
/// * `obj` - A `DefaultDicomObject` representing the DICOM object to be converted.
///
/// # Returns
///
/// * `Ok(RTPlan)` - If the conversion is successful, returns an `RTPlan` object containing
///   the parsed information from the DICOM object.
/// * `Err(DcmIOError)` - If an error occurs during the conversion process.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The DICOM object does not contain a valid RT Plan SOP Class UID.
/// * Required fields are missing or have invalid formats.
/// * Unexpected errors occur during the parsing of specific DICOM elements.
///
/// # Examples
///
/// ```
/// use dicom_object::DefaultDicomObject;
/// use rad_tools_dcm_data::io::obj_to_rtplan;
///
/// let dicom_obj = DefaultDicomObject::open_file("tests/resources/RP1.2.752.243.1.1.20220722130644612.2000.30831.dcm").unwrap();
/// let rt_plan = obj_to_rtplan(dicom_obj);
/// match rt_plan {
///     Ok(plan) => println!("Successfully converted DICOM object to RT Plan: {:?}", plan),
///     Err(e) => eprintln!("Failed to convert DICOM object: {}", e),
/// }
/// ```
///
/// # Dependencies
///
/// This function internally relies on the `dicom-object` crate to handle DICOM objects.
///
pub fn obj_to_rtplan(obj: DefaultDicomObject) -> Result<RTPlan, DcmIOError> {
    let obj = obj.into_inner();
    let sop_class_uid = to_string(&obj, SOP_CLASS_UID)?;
    if sop_class_uid != RT_PLAN_STORAGE {
        return Err(DcmIOError::NoMatchingSopClassUID(sop_class_uid));
    }
    Ok(RTPlan {
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
        series_number: to_int_opt(&obj, SERIES_NUMBER)?,
        frame_of_reference_uid: to_string(&obj, FRAME_OF_REFERENCE_UID)?,
        position_reference_indicator: to_string_opt(&obj, POSITION_REFERENCE_INDICATOR)?,
        rt_plan_label: to_string(&obj, RT_PLAN_LABEL)?,
        rt_plan_name: to_string_opt(&obj, RT_PLAN_NAME)?,
        rt_plan_description: to_string_opt(&obj, RT_PLAN_DESCRIPTION)?,
        rt_plan_dt: da_tm_to_ndt_opt(&obj, RT_PLAN_DATE, RT_PLAN_TIME)?,
        treatment_protocols: to_string_opt(&obj, TREATMENT_PROTOCOLS)?,
        plan_intent: to_string_opt(&obj, PLAN_INTENT)?,
        rt_plan_geometry: to_string(&obj, RT_PLAN_GEOMETRY)?,
        fraction_group_sequence: from_seq(&obj, FRACTION_GROUP_SEQUENCE, fraction_group)?,
        beam_sequence: from_seq(&obj, BEAM_SEQUENCE, beam)?,
        patient_setup_sequence: from_seq(&obj, PATIENT_SETUP_SEQUENCE, patient_setup)?,
        referenced_structure_set_sequence: from_seq(
            &obj,
            REFERENCED_STRUCTURE_SET_SEQUENCE,
            referenced_structure_set,
        )?,
        approval_status: to_string_opt(&obj, APPROVAL_STATUS)?
            .map(|s| ApprovalStatus::from_str(&s).unwrap()),
        review_dt: da_tm_to_ndt_opt(&obj, REVIEW_DATE, REVIEW_TIME)?,
        reviewer_name: to_string_opt(&obj, REVIEWER_NAME)?
            .map(|s| PersonName::from_str(&s).unwrap()),
    })
}

fn fraction_group(item: &InMemDicomObject) -> Result<FractionGroup, DcmIOError> {
    Ok(FractionGroup {
        fraction_group_number: to_int(item, FRACTION_GROUP_NUMBER)?,
        fraction_group_description: to_string_opt(item, FRACTION_GROUP_DESCRIPTION)?,
        number_of_fractions_planned: to_int_opt(item, NUMBER_OF_FRACTIONS_PLANNED)?,
        number_of_fraction_pattern_digits_per_day: to_int_opt(
            item,
            NUMBER_OF_FRACTION_PATTERN_DIGITS_PER_DAY,
        )?,
        repeat_fraction_cycle_length: to_int_opt(item, REPEAT_FRACTION_CYCLE_LENGTH)?,
        fraction_pattern: to_string_opt(item, FRACTION_PATTERN)?,
        number_of_beams: to_int(item, NUMBER_OF_BEAMS)?,
        beam_dose_meaning: to_string_opt(item, BEAM_DOSE_MEANING)?,
        number_of_brachy_application_setups: to_int(item, NUMBER_OF_BRACHY_APPLICATION_SETUPS)?,
        referenced_beam_sequence: from_seq(item, REFERENCED_BEAM_SEQUENCE, referenced_beam)?,
        referenced_brachy_application_setup_sequence: from_seq_opt(
            item,
            REFERENCED_BRACHY_APPLICATION_SETUP_SEQUENCE,
            referenced_brachy_application_setup,
        )?,
        referenced_dose_reference_sequence: from_seq_opt(
            item,
            REFERENCED_DOSE_REFERENCE_SEQUENCE,
            referenced_dose_reference,
        )?,
        referenced_dose_sequence: from_seq_opt(item, REFERENCED_DOSE_SEQUENCE, referenced_sop)?,
    })
}

fn referenced_beam(item: &InMemDicomObject) -> Result<ReferencedBeam, DcmIOError> {
    Ok(ReferencedBeam {
        referenced_dose_reference_uid: to_string_opt(item, REFERENCED_DOSE_REFERENCE_UID)?,
        beam_dose: to_f64_opt(item, BEAM_DOSE)?,
        beam_meterset: to_f64_opt(item, BEAM_METERSET)?,
        beam_dose_point_depth: to_f64_opt(item, BEAM_DOSE_POINT_DEPTH)?,
        beam_dose_point_equivalent_depth: to_f64_opt(item, BEAM_DOSE_POINT_EQUIVALENT_DEPTH)?,
        beam_dose_point_ssd: to_f64_opt(item, BEAM_DOSE_POINT_SSD)?,
        beam_dose_type: to_string_opt(item, BEAM_DOSE_TYPE)?
            .map(|s| BeamDoseType::from_str(&s).unwrap()),
        referenced_beam_number: to_int(item, REFERENCED_BEAM_NUMBER)?,
    })
}

fn referenced_brachy_application_setup(
    item: &InMemDicomObject,
) -> Result<ReferencedBrachyApplicationSetup, DcmIOError> {
    Ok(ReferencedBrachyApplicationSetup {
        referenced_dose_reference_uid: to_string_opt(item, REFERENCED_DOSE_REFERENCE_UID)?,
        brachy_application_setup_dose_specification_point: to_f64s_opt(
            item,
            BRACHY_APPLICATION_SETUP_DOSE_SPECIFICATION_POINT,
        )?,
        brachy_application_setup_dose: to_f64_opt(item, BRACHY_APPLICATION_SETUP_DOSE)?,
        referenced_brachy_application_setup_number: to_int(
            item,
            REFERENCED_BRACHY_APPLICATION_SETUP_NUMBER,
        )?,
    })
}

fn referenced_dose_reference(
    item: &InMemDicomObject,
) -> Result<ReferencedDoseReference, DcmIOError> {
    Ok(ReferencedDoseReference {
        constraint_weight: to_f64_opt(item, CONSTRAINT_WEIGHT)?,
        delivery_warning_dose: to_f64_opt(item, DELIVERY_WARNING_DOSE)?,
        delivery_maximum_dose: to_f64_opt(item, DELIVERY_MAXIMUM_DOSE)?,
        target_minimum_dose: to_f64_opt(item, TARGET_MINIMUM_DOSE)?,
        target_prescription_dose: to_f64_opt(item, TARGET_PRESCRIPTION_DOSE)?,
        target_underdose_volume_fraction: to_f64_opt(item, TARGET_UNDERDOSE_VOLUME_FRACTION)?,
        organ_at_risk_full_volume_dose: to_f64_opt(item, ORGAN_AT_RISK_FULL_VOLUME_DOSE)?,
        organ_at_risk_limit_dose: to_f64_opt(item, ORGAN_AT_RISK_LIMIT_DOSE)?,
        organ_at_risk_maximum_dose: to_f64_opt(item, ORGAN_AT_RISK_MAXIMUM_DOSE)?,
        organ_at_risk_overdose_volume_fraction: to_f64_opt(
            item,
            ORGAN_AT_RISK_OVERDOSE_VOLUME_FRACTION,
        )?,
        referenced_dose_reference_number: to_int_opt(item, REFERENCED_DOSE_REFERENCE_NUMBER)?,
    })
}

fn beam(item: &InMemDicomObject) -> Result<Beam, DcmIOError> {
    Ok(Beam {
        primary_fluence_mode_sequence: from_seq_opt(
            item,
            PRIMARY_FLUENCE_MODE_SEQUENCE,
            primary_fluence_mode,
        )?,
        treatment_machine_name: to_string_opt(item, TREATMENT_MACHINE_NAME)?,
        primary_dosimeter_unit: to_string_opt(item, PRIMARY_DOSIMETER_UNIT)?
            .map(|s| PrimaryDosimeterUnit::from_str(&s).unwrap()),
        source_axis_distance: to_f64_opt(item, SOURCE_AXIS_DISTANCE)?,
        beam_limiting_device_sequence: from_seq(
            item,
            BEAM_LIMITING_DEVICE_SEQUENCE,
            beam_limiting_device,
        )?,
        beam_number: to_int(item, BEAM_NUMBER)?,
        beam_type: to_string_opt(item, BEAM_TYPE)?.map(|s| BeamType::from_str(&s).unwrap()),
        beam_name: to_string_opt(item, BEAM_NAME)?,
        beam_description: to_string_opt(item, BEAM_DESCRIPTION)?,
        radiation_type: to_string_opt(item, RADIATION_TYPE)?
            .map(|s| RadiationType::from_str(&s).unwrap()),
        treatment_delivery_type: to_string_opt(item, TREATMENT_DELIVERY_TYPE)?
            .map(|s| TreatmentDeliveryType::from_str(&s).unwrap()),
        number_of_wedges: to_int(item, NUMBER_OF_WEDGES)?,
        number_of_compensators: to_int(item, NUMBER_OF_COMPENSATORS)?,
        number_of_boli: to_int(item, NUMBER_OF_BOLI)?,
        number_of_blocks: to_int(item, NUMBER_OF_BLOCKS)?,
        final_cumulative_meterset_weight: to_f64(item, FINAL_CUMULATIVE_METERSET_WEIGHT)?,
        number_of_control_points: to_int(item, NUMBER_OF_CONTROL_POINTS)?,
        control_point_sequence: from_seq(item, CONTROL_POINT_SEQUENCE, control_point)?,
        referenced_patient_setup_number: to_int_opt(item, REFERENCED_PATIENT_SETUP_NUMBER)?,
        referenced_bolus_sequence: from_seq_opt(item, REFERENCED_BOLUS_SEQUENCE, referenced_bolus)?,
    })
}

fn primary_fluence_mode(item: &InMemDicomObject) -> Result<PrimaryFluenceMode, DcmIOError> {
    Ok(PrimaryFluenceMode {
        fluence_mode: FluenceMode::from_str(&to_string(item, FLUENCE_MODE)?)?,
        fluence_mode_id: to_string_opt(item, FLUENCE_MODE_ID)?,
    })
}

fn beam_limiting_device(item: &InMemDicomObject) -> Result<BeamLimitingDevice, DcmIOError> {
    Ok(BeamLimitingDevice {
        rt_beam_limiting_device_type: RTBeamLimitingDeviceType::from_str(&to_string(
            item,
            RT_BEAM_LIMITING_DEVICE_TYPE,
        )?)?,
        source_to_beam_limiting_device_distance: to_f64_opt(
            item,
            SOURCE_TO_BEAM_LIMITING_DEVICE_DISTANCE,
        )?,
        number_of_leaf_jaw_pairs: to_int(item, NUMBER_OF_LEAF_JAW_PAIRS)?,
        leaf_position_boundaries: to_f64s_opt(item, LEAF_POSITION_BOUNDARIES)?,
    })
}

fn control_point(item: &InMemDicomObject) -> Result<ControlPoint, DcmIOError> {
    Ok(ControlPoint {
        control_point_index: to_int(item, CONTROL_POINT_INDEX)?,
        nominal_beam_energy: to_f64_opt(item, NOMINAL_BEAM_ENERGY)?,
        beam_limiting_device_position_sequence: from_seq_opt(
            item,
            BEAM_LIMITING_DEVICE_POSITION_SEQUENCE,
            beam_limiting_device_position,
        )?,
        gantry_angle: to_f64_opt(item, GANTRY_ANGLE)?,
        gantry_rotation_direction: to_rotation_direction_opt(item, GANTRY_ROTATION_DIRECTION)?,
        beam_limiting_device_angle: to_f64_opt(item, BEAM_LIMITING_DEVICE_ANGLE)?,
        beam_limiting_device_rotation_direction: to_rotation_direction_opt(
            item,
            BEAM_LIMITING_DEVICE_ROTATION_DIRECTION,
        )?,
        patient_support_angle: to_f64_opt(item, PATIENT_SUPPORT_ANGLE)?,
        patient_support_rotation_direction: to_rotation_direction_opt(
            item,
            PATIENT_SUPPORT_ROTATION_DIRECTION,
        )?,
        table_top_eccentric_angle: to_f64_opt(item, TABLE_TOP_ECCENTRIC_ANGLE)?,
        table_top_eccentric_rotation_direction: to_rotation_direction_opt(
            item,
            TABLE_TOP_ECCENTRIC_ROTATION_DIRECTION,
        )?,
        table_top_vertical_position: to_f64_opt(item, TABLE_TOP_VERTICAL_POSITION)?,
        table_top_longitudinal_position: to_f64_opt(item, TABLE_TOP_LONGITUDINAL_POSITION)?,
        table_top_lateral_position: to_f64_opt(item, TABLE_TOP_LATERAL_POSITION)?,
        isocenter_position: to_f64s_opt(item, ISOCENTER_POSITION)?,
        source_to_surface_distance: to_f64_opt(item, SOURCE_TO_SURFACE_DISTANCE)?,
        cumulative_meterset_weight: to_f64_opt(item, CUMULATIVE_METERSET_WEIGHT)?,
        table_top_pitch_angle: to_f64_opt(item, TABLE_TOP_PITCH_ANGLE)?,
        table_top_pitch_rotation_direction: to_rotation_direction_opt(
            item,
            TABLE_TOP_PITCH_ROTATION_DIRECTION,
        )?,
        table_top_roll_angle: to_f64_opt(item, TABLE_TOP_ROLL_ANGLE)?,
        table_top_roll_rotation_direction: to_rotation_direction_opt(
            item,
            TABLE_TOP_ROLL_ROTATION_DIRECTION,
        )?,
        gantry_pitch_angle: to_f64_opt(item, GANTRY_PITCH_ANGLE)?,
        gantry_pitch_rotation_direction: to_rotation_direction_opt(
            item,
            GANTRY_PITCH_ROTATION_DIRECTION,
        )?,
    })
}

fn beam_limiting_device_position(
    item: &InMemDicomObject,
) -> Result<BeamLimitingDevicePosition, DcmIOError> {
    Ok(BeamLimitingDevicePosition {
        rt_beam_limiting_device_type: RTBeamLimitingDeviceType::from_str(&to_string(
            item,
            RT_BEAM_LIMITING_DEVICE_TYPE,
        )?)?,
        leaf_jaw_positions: to_f64s(item, LEAF_JAW_POSITIONS)?,
    })
}

fn referenced_bolus(item: &InMemDicomObject) -> Result<ReferencedBolus, DcmIOError> {
    Ok(ReferencedBolus {
        referenced_roi_number: to_int(item, REFERENCED_ROI_NUMBER)?,
        bolus_id: to_string_opt(item, BOLUS_ID)?,
        bolus_description: to_string_opt(item, BOLUS_DESCRIPTION)?,
        accessory_code: to_string_opt(item, ACCESSORY_CODE)?,
    })
}

fn patient_setup(item: &InMemDicomObject) -> Result<PatientSetup, DcmIOError> {
    Ok(PatientSetup {
        patient_position: PatientPosition::from_str(&to_string(item, PATIENT_POSITION)?)?,
        patient_setup_number: to_int(item, PATIENT_SETUP_NUMBER)?,
    })
}

fn referenced_structure_set(item: &InMemDicomObject) -> Result<Sop, DcmIOError> {
    read_sop(item, REFERENCED_SOP_CLASS_UID, REFERENCED_SOP_INSTANCE_UID)
}
