use crate::io::common::read_sop;
use crate::io::{da_tm_to_ndt, da_tm_to_ndt_opt, from_seq, to_date, to_f64, to_f64_opt, to_f64s, to_int, to_int_opt, to_string, to_string_opt, DcmIOError};
use crate::{CodeItem, PatientPosition, PersonName, PhotometricInterpretation, PixelRepresentation, RescaleType, RotationDirection, CT};
use dicom_dictionary_std::tags::{ACCESSION_NUMBER, ACQUISITION_NUMBER, BITS_ALLOCATED, BITS_STORED, BODY_PART_EXAMINED, BURNED_IN_ANNOTATION, CODE_MEANING, CODE_VALUE, CODING_SCHEME_DESIGNATOR, COLUMNS, CONTENT_DATE, CONTENT_TIME, CONVOLUTION_KERNEL, CTDI_PHANTOM_TYPE_CODE_SEQUENCE, CTD_IVOL, DATA_COLLECTION_CENTER_PATIENT, DATA_COLLECTION_DIAMETER, DATE_OF_LAST_CALIBRATION, DEVICE_SERIAL_NUMBER, DISTANCE_SOURCE_TO_DETECTOR, DISTANCE_SOURCE_TO_PATIENT, EXPOSURE, EXPOSURE_MODULATION_TYPE, EXPOSURE_TIME, FILTER_TYPE, FOCAL_SPOTS, FRAME_OF_REFERENCE_UID, GANTRY_DETECTOR_TILT, HIGH_BIT, IMAGE_ORIENTATION_PATIENT, IMAGE_POSITION_PATIENT, IMAGE_TYPE, INSTANCE_NUMBER, IRRADIATION_EVENT_UID, KVP, LARGEST_IMAGE_PIXEL_VALUE, LOSSY_IMAGE_COMPRESSION, MANUFACTURER, MANUFACTURER_MODEL_NAME, MODALITY, PATIENT_BIRTH_DATE, PATIENT_ID, PATIENT_IDENTITY_REMOVED, PATIENT_NAME, PATIENT_ORIENTATION, PATIENT_POSITION, PATIENT_SEX, PHOTOMETRIC_INTERPRETATION, PIXEL_DATA, PIXEL_PADDING_VALUE, PIXEL_REPRESENTATION, PIXEL_SPACING, PLANAR_CONFIGURATION, POSITION_REFERENCE_INDICATOR, RECONSTRUCTION_DIAMETER, RECONSTRUCTION_TARGET_CENTER_PATIENT, REFERRING_PHYSICIAN_NAME, RESCALE_INTERCEPT, RESCALE_SLOPE, RESCALE_TYPE, REVOLUTION_TIME, ROTATION_DIRECTION, ROWS, SAMPLES_PER_PIXEL, SERIES_DATE, SERIES_DESCRIPTION, SERIES_INSTANCE_UID, SERIES_NUMBER, SERIES_TIME, SINGLE_COLLIMATION_WIDTH, SLICE_LOCATION, SLICE_THICKNESS, SMALLEST_IMAGE_PIXEL_VALUE, SOFTWARE_VERSIONS, SOP_CLASS_UID, SOP_INSTANCE_UID, SPECIFIC_CHARACTER_SET, SPIRAL_PITCH_FACTOR, STATION_NAME, STUDY_DATE, STUDY_DESCRIPTION, STUDY_ID, STUDY_INSTANCE_UID, STUDY_TIME, TABLE_FEED_PER_ROTATION, TABLE_HEIGHT, TABLE_SPEED, TIME_OF_LAST_CALIBRATION, TOTAL_COLLIMATION_WIDTH, WINDOW_CENTER, WINDOW_CENTER_WIDTH_EXPLANATION, WINDOW_WIDTH, X_RAY_TUBE_CURRENT};
use dicom_object::InMemDicomObject;
use std::path::Path;
use std::str::FromStr;


/// Reads a CT image from a specified file path and returns a `CT` object containing the parsed DICOM attributes.
///
/// # Arguments
///
/// * `path` - A file path to the DICOM file to read.
///
/// # Returns
///
/// A `Result` which is:
/// * `Ok(CT)` containing the CT image data on success.
/// * `Err(DcmIOError)` if any error occurs during reading or parsing the DICOM file.
/// ```
pub fn read_ct_image<P: AsRef<Path>>(path: P) -> Result<CT, DcmIOError> {
    let obj = dicom_object::open_file(path.as_ref())?.into_inner();
    let mut ct = CT::default();

    ct.specific_character_set = to_string(&obj, SPECIFIC_CHARACTER_SET)?;

    ct.image_type = to_string(&obj, IMAGE_TYPE)?
        .split('\\')
        .map(|x| x.to_string())
        .collect();
    ct.sop = read_sop(&obj, SOP_CLASS_UID, SOP_INSTANCE_UID)?;
    ct.study_dt = da_tm_to_ndt_opt(&obj, STUDY_DATE, STUDY_TIME)?;
    ct.series_dt = da_tm_to_ndt_opt(&obj, SERIES_DATE, SERIES_TIME)?;
    ct.content_dt = da_tm_to_ndt_opt(&obj, CONTENT_DATE, CONTENT_TIME)?;
    ct.accession_number = to_string_opt(&obj, ACCESSION_NUMBER)?;
    ct.modality = to_string(&obj, MODALITY)?;
    ct.ref_physician_name = match to_string_opt(&obj, REFERRING_PHYSICIAN_NAME)? {
        Some(s) => Some(PersonName::from_str(&s).unwrap()),
        None => None,
    };
    ct.station_name = to_string_opt(&obj, STATION_NAME)?;
    ct.study_description = to_string_opt(&obj, STUDY_DESCRIPTION)?;
    ct.series_description = to_string_opt(&obj, SERIES_DESCRIPTION)?;
    ct.manufacturer= to_string_opt(&obj, MANUFACTURER)?;
    ct.manufacturer_model_name = to_string_opt(&obj, MANUFACTURER_MODEL_NAME)?;
    ct.irradiation_event_uid = to_string(&obj, IRRADIATION_EVENT_UID)?;
    ct.patient_name = PersonName::from_str(&to_string(&obj, PATIENT_NAME)?).unwrap();
    ct.patient_id = to_string(&obj, PATIENT_ID)?;
    ct.patient_birth_date = to_date(&obj, PATIENT_BIRTH_DATE)?;
    ct.patient_sex = to_string(&obj, PATIENT_SEX)?;
    ct.patient_identity_removed = if to_string(&obj, PATIENT_IDENTITY_REMOVED)? == "YES" {
        true
    } else {
        false
    };
    ct.body_part_examined = to_string(&obj, BODY_PART_EXAMINED)?;
    ct.slice_thickness = to_f64_opt(&obj, SLICE_THICKNESS)?;
    ct.kvp = to_f64(&obj, KVP)?;
    ct.data_collection_diameter = to_f64(&obj, DATA_COLLECTION_DIAMETER)?;
    ct.device_serial_number = to_string(&obj, DEVICE_SERIAL_NUMBER)?;
    ct.software_versions = to_string(&obj, SOFTWARE_VERSIONS)?;
    ct.reconstruction_diameter = to_f64(&obj, RECONSTRUCTION_DIAMETER)?;
    ct.distance_source_to_detector = to_f64(&obj, DISTANCE_SOURCE_TO_DETECTOR)?;
    ct.distance_source_to_patient = to_f64(&obj, DISTANCE_SOURCE_TO_PATIENT)?;
    ct.gantry_detector_tilt = to_f64(&obj, GANTRY_DETECTOR_TILT)?;
    ct.table_height = to_f64(&obj, TABLE_HEIGHT)?;
    ct.rotation_direction = RotationDirection::from_str(&to_string(&obj, ROTATION_DIRECTION)?)?;
    ct.exposure_time = to_int(&obj, EXPOSURE_TIME)?;
    ct.xray_tube_current = to_int(&obj, X_RAY_TUBE_CURRENT)?;
    ct.exposure = to_int(&obj, EXPOSURE)?;
    ct.filter_type = to_string(&obj, FILTER_TYPE)?;
    ct.genereator_power = to_int(&obj, GANTRY_DETECTOR_TILT)?;
    ct.focal_spots = match obj.element(FOCAL_SPOTS)?.to_multi_float64() {
        Ok(v) => {
            if v.len() == 2 {
                Ok([v[0], v[1]])
            } else {
                Err(DcmIOError::InvalidFocalSpots(v.clone()))
            }
        }
        Err(e) => Err(DcmIOError::from(e)),
    }?;
    ct.last_calibration_dt =
        da_tm_to_ndt(&obj, DATE_OF_LAST_CALIBRATION, TIME_OF_LAST_CALIBRATION)?;
    ct.pixel_padding_value = to_int(&obj, PIXEL_PADDING_VALUE)?;
    ct.convolution_kernel = to_string(&obj, CONVOLUTION_KERNEL)?;
    ct.patient_position = PatientPosition::from_str(&to_string(&obj, PATIENT_POSITION)?)?;
    ct.revolution_time = to_f64(&obj, REVOLUTION_TIME)?;
    ct.single_collimation_width = to_f64(&obj, SINGLE_COLLIMATION_WIDTH)?;
    ct.total_collimation_width = to_f64(&obj, TOTAL_COLLIMATION_WIDTH)?;
    ct.table_speed = to_f64(&obj, TABLE_SPEED)?;
    ct.table_feed_per_rotation = to_f64(&obj, TABLE_FEED_PER_ROTATION)?;
    ct.spiral_pitch_factor = to_f64(&obj, SPIRAL_PITCH_FACTOR)?;
    ct.data_collection_center_patient = match obj
        .element(DATA_COLLECTION_CENTER_PATIENT)?
        .to_multi_float64()
    {
        Ok(v) => {
            if v.len() == 3 {
                Ok([v[0], v[1], v[2]])
            } else {
                Err(DcmIOError::InvalidDataCollectionCenterPatient(v.clone()))
            }
        }
        Err(e) => Err(DcmIOError::from(e)),
    }?;
    ct.reconstruction_target_center_patient = match obj
        .element(RECONSTRUCTION_TARGET_CENTER_PATIENT)?
        .to_multi_float64()
    {
        Ok(v) => {
            if v.len() == 3 {
                Ok([v[0], v[1], v[2]])
            } else {
                Err(DcmIOError::InvalidReconstructionTargetCenterPatient(
                    v.clone(),
                ))
            }
        }
        Err(e) => Err(DcmIOError::from(e)),
    }?;
    ct.exposure_modulation_type = to_string(&obj, EXPOSURE_MODULATION_TYPE)?;
    ct.ctdi_vol = to_f64(&obj, CTD_IVOL)?;
    ct.ctdi_phantom_type_code_sequence = from_seq(
        &obj,
        CTDI_PHANTOM_TYPE_CODE_SEQUENCE,
        ctdi_phantom_type_code,
    )?;
    ct.study_instance_uid = to_string(&obj, STUDY_INSTANCE_UID)?;
    ct.patient_orientation = to_string_opt(&obj, PATIENT_ORIENTATION)?;
    ct.series_instance_uid = to_string(&obj, SERIES_INSTANCE_UID)?;
    ct.study_id = to_string_opt(&obj, STUDY_ID)?;
    ct.series_number = to_int(&obj, SERIES_NUMBER)?;
    ct.acquisition_number = to_int(&obj, ACQUISITION_NUMBER)?;
    ct.instance_number = to_int(&obj, INSTANCE_NUMBER)?;
    ct.image_position_patient = match to_f64s(&obj, IMAGE_POSITION_PATIENT) {
        Ok(v) => {
            if v.len() == 3 {
                Ok([v[0], v[1], v[2]])
            } else {
                Err(DcmIOError::InvalidImagePositionPatient(v.clone()))
            }
        }
        Err(e) => { Err(e)}
    }?;
    ct.image_orientation_patient = match to_f64s(&obj, IMAGE_ORIENTATION_PATIENT){
        Ok(v) => {
            if v.len() == 6 {
                Ok([v[0], v[1], v[2], v[3], v[4], v[5]])
            } else {
                Err(DcmIOError::InvalidImageOrientationPatient(v.clone()))
            }
        }
        Err(e) => { Err(e)}
    }?;
    ct.frame_of_reference_uid = to_string(&obj, FRAME_OF_REFERENCE_UID)?;
    ct.position_reference_indicator = to_string_opt(&obj, POSITION_REFERENCE_INDICATOR)?;
    ct.slice_location = to_f64_opt(&obj, SLICE_LOCATION)?;
    ct.samples_per_pixel = to_int(&obj, SAMPLES_PER_PIXEL)?;
    ct.photometric_interpretation = PhotometricInterpretation::from_str(&to_string(&obj, PHOTOMETRIC_INTERPRETATION)?)?;
    ct.planar_configuration = to_int_opt(&obj, PLANAR_CONFIGURATION)?;
    ct.rows = to_int(&obj, ROWS)?;
    ct.columns = to_int(&obj, COLUMNS)?;
    ct.pixel_spacing = match to_f64s(&obj, PIXEL_SPACING){
        Ok(v) => {
            if v.len() == 2 {
                Ok([v[0], v[1]])
            } else {
                Err(DcmIOError::InvalidPixelSpacing(v.clone()))
            }
        }
        Err(e) => { Err(e)}
    }?;
    ct.bits_allocated = to_int(&obj, BITS_ALLOCATED)?;
    ct.bits_stored = to_int(&obj, BITS_STORED)?;
    ct.high_bit = to_int(&obj, HIGH_BIT)?;
    ct.pixel_representation = PixelRepresentation::from_str(&to_string(&obj, PIXEL_REPRESENTATION)?)?;
    ct.smallest_image_pixel_value = to_int_opt(&obj, SMALLEST_IMAGE_PIXEL_VALUE)?;
    ct.largest_image_pixel_value = to_int_opt(&obj, LARGEST_IMAGE_PIXEL_VALUE)?;
    ct.burned_in_annotation = to_string_opt(&obj, BURNED_IN_ANNOTATION)?;
    ct.window_center = to_f64(&obj, WINDOW_CENTER)?;
    ct.window_width = to_f64(&obj, WINDOW_WIDTH)?;
    ct.rescale_intercept = to_f64(&obj, RESCALE_INTERCEPT)?;
    ct.rescale_slope = to_f64(&obj, RESCALE_SLOPE)?;
    ct.rescale_type = RescaleType::from_str(&to_string(&obj, RESCALE_TYPE)?)?;
    ct.window_center_width_explanation = to_string_opt(&obj, WINDOW_CENTER_WIDTH_EXPLANATION)?;
    ct.lossy_image_compression = to_string_opt(&obj, LOSSY_IMAGE_COMPRESSION)?;
    ct.pixel_data = obj.element(PIXEL_DATA)?.to_bytes()?.to_vec();
    Ok(ct)
}


/// Constructs a `CodeItem` from the provided `InMemDicomObject` reference.
///
/// This function extracts the `code_value`, `coding_scheme_designator`, and 
/// `code_meaning` from the given DICOM object and uses them to populate a `CodeItem`.
/// It returns the resulting `CodeItem` or an error if any of the field extraction fails.
///
/// # Arguments
///
/// * `item` - A reference to an `InMemDicomObject` from which the code information
///            is to be extracted.
///
/// # Returns
///
/// * `Ok(CodeItem)` - If the extraction of all required fields is successful.
/// * `Err(DcmIOError)` - If any of the required fields cannot be extracted.
///
/// # Errors
///
/// This function returns a `DcmIOError` if any of the following field extractions fail:
/// - `code_value`
/// - `coding_scheme_designator`
/// - `code_meaning`
///
fn ctdi_phantom_type_code(item: &InMemDicomObject) -> Result<CodeItem, DcmIOError> {
    let mut code_item = CodeItem::default();
    code_item.code_value = to_string(item, CODE_VALUE)?;
    code_item.coding_scheme_designator = to_string(item, CODING_SCHEME_DESIGNATOR)?;
    code_item.code_meaning = to_string(item, CODE_MEANING)?;
    Ok(code_item)
}
