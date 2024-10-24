use crate::{CodeItem, PatientPosition, PersonName, PhotometricInterpretation, PixelRepresentation, RotationDirection, Sop};
use chrono::NaiveDateTime;

#[derive(Clone, Debug, Default, )]
pub struct CT {
    pub specific_character_set: String,
    pub image_type: Vec<String>,
    pub sop: Sop,
    pub study_dt: NaiveDateTime,
    pub content_dt: NaiveDateTime,
    pub accession_number: String,
    pub modality: String,
    pub ref_physician_name: PersonName,
    pub station_name: String,
    pub manufacturer_model_name: String,
    pub irradiation_event_uid: String,
    pub patient_name: PersonName,
    pub patient_id: String,
    pub patient_birth_date: NaiveDateTime,
    pub patient_sex: String,
    pub patient_identity_removed: String,
    pub body_part_examined: String,
    pub slice_thickness: f64,
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
    pub last_calibration_dt: NaiveDateTime,
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
    pub study_id: String,
    pub series_number: u32,
    pub acquisition_number: u32,
    pub instance_number: u32,
    pub image_position_patient: [f64; 3],
    pub image_orientation_patient: [f64; 6],
    pub frame_of_reference_uid: String,
    pub position_reference_indicator: String,
    pub slice_location: f64,
    pub samples_per_pixel: u32,
    pub photometric_interpretation: PhotometricInterpretation,
    pub rows: u32,
    pub columns: u32,
    pub pixel_spacing: [f64; 2],
    pub bits_allocated: u32,
    pub bits_stored: u32,
    pub high_bit: u32,
    pub pixel_representation: PixelRepresentation,
    pub smallest_image_pixel_value: u32,
    pub largest_image_pixel_value: u32,
    pub burned_in_annotation: String,
    pub window_center: f64,
    pub window_width: f64,
    pub rescale_intercept: f64,
    pub rescale_slope: f64,
    pub rescale_type: String,
    pub window_center_width_explanation: String,
    pub lossy_image_compression: String,
    pub pixel_data: Vec<u8>,
}