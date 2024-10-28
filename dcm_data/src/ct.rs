use crate::{CodeItem, Modality, PatientPosition, PersonName, PhotometricInterpretation, PixelRepresentation, RescaleType, RotationDirection, Sop};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Clone, Debug, Default, )]
pub struct CT {
    pub specific_character_set: String,
    pub image_type: Vec<String>,
    pub sop: Sop,
    pub study_dt: Option<NaiveDateTime>,
    pub series_dt: Option<NaiveDateTime>,
    pub content_dt: Option<NaiveDateTime>,
    pub accession_number: Option<String>,
    pub modality: Modality,
    pub ref_physician_name: Option<PersonName>,
    pub station_name: Option<String>,
    pub study_description: Option<String>,
    pub series_description: Option<String>,
    pub manufacturer: Option<String>,
    pub manufacturer_model_name: Option<String>,
    pub irradiation_event_uid: String,
    pub patient_name: PersonName,
    pub patient_id: String,
    pub patient_birth_date: NaiveDate,
    pub patient_sex: String,
    pub patient_identity_removed: bool,
    pub body_part_examined: String,
    pub slice_thickness: Option<f64>,
    pub kvp: f64,
    pub data_collection_diameter: f64,
    pub device_serial_number: String,
    pub software_versions: String,
    pub reconstruction_diameter: f64,
    pub distance_source_to_detector: f64,
    pub distance_source_to_patient: f64,
    pub gantry_detector_tilt: f64,
    pub table_height: f64,
    pub rotation_direction: RotationDirection,
    pub exposure_time: u32,
    pub xray_tube_current: u32,
    pub exposure: u32,
    pub filter_type: String,
    pub genereator_power: u32,
    pub focal_spots: [f64; 2],
    pub last_calibration_dt: Option<NaiveDateTime>,
    pub pixel_padding_value: Option<i32>,
    pub convolution_kernel: String,
    pub patient_position: PatientPosition,
    pub revolution_time: f64,
    pub single_collimation_width: f64,
    pub total_collimation_width: f64,
    pub table_speed: f64,
    pub table_feed_per_rotation: f64,
    pub spiral_pitch_factor: f64,
    pub data_collection_center_patient: [f64; 3],
    pub reconstruction_target_center_patient: [f64; 3],
    pub exposure_modulation_type: String,
    pub ctdi_vol: f64,
    pub ctdi_phantom_type_code_sequence: Vec<CodeItem>,
    pub study_instance_uid: String,
    pub series_instance_uid: String,
    pub study_id: Option<String>,
    pub series_number: u32,
    pub acquisition_number: u32,
    pub instance_number: u32,
    pub patient_orientation: Option<String>,
    pub image_position_patient: [f64; 3],
    pub image_orientation_patient: [f64; 6],
    pub frame_of_reference_uid: String,
    pub position_reference_indicator: Option<String>,
    pub slice_location: Option<f64>,
    pub samples_per_pixel: u16,
    pub photometric_interpretation: PhotometricInterpretation,
    pub planar_configuration: Option<u16>,
    pub rows: u16,
    pub columns: u16,
    pub pixel_spacing: [f64; 2],
    pub bits_allocated: u16,
    pub bits_stored: u16,
    pub high_bit: u16,
    pub pixel_representation: PixelRepresentation,
    pub smallest_image_pixel_value: Option<u32>,
    pub largest_image_pixel_value: Option<u32>,
    pub burned_in_annotation: Option<String>,
    pub window_center: f64,
    pub window_width: f64,
    pub rescale_intercept: f64,
    pub rescale_slope: f64,
    pub rescale_type: RescaleType,
    pub window_center_width_explanation: Option<String>,
    pub lossy_image_compression: Option<String>,
    pub pixel_data: Vec<u8>,
}