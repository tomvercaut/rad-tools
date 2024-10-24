use crate::{PersonName, Sop};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Clone, Debug, Default, )]
pub struct RTStruct {
    pub specific_character_set: String,
    pub instance_creation_dt: NaiveDateTime,
    pub sop: Sop,
    pub study_dt: NaiveDateTime,
    pub accession_number: String,
    pub modality: String,
    pub manufacturer: String,
    pub referring_physician_name: PersonName,
    pub manufacturer_model_name: String,
    pub patient_name: PersonName,
    pub patient_id: String,
    pub patient_birth_date: NaiveDate,
    pub patient_sex: String,
    pub patient_identity_removed: bool,
    pub deidentification_method: String,
    pub software_versions: String,
    pub study_instance_uid: String,
    pub series_instance_uid: String,
    pub study_id: String,
    pub series_number: i32,
    pub instance_number: i32,
    pub frame_of_reference_uid: String,
    pub position_reference_indicator: String,
    pub structure_set_label: String,
    pub structure_set_dt: NaiveDateTime,
    pub referenced_frame_of_reference_seq: Vec<ReferencedFrameOfReference>,
    pub structure_set_roi_sequence: Vec<StructureSetROI>,
    pub roi_contour_sequence: Vec<RoiContour>,
    pub rt_roi_observations_sequence: Vec<RTRoiObservation>,
}

#[derive(Clone, Debug, Default, )]
pub struct ReferencedFrameOfReference {
    pub frame_of_reference_uid: String,
    pub frame_of_reference_relationship_sequence: Vec<RTReferencedStudy>,
}

#[derive(Clone, Debug, Default, )]
pub struct RTReferencedStudy {
    pub referenced_sop: Sop,
    pub rt_referenced_series_sequence: Vec<RTReferencedSerie>,
}

#[derive(Clone, Debug, Default, )]
pub struct RTReferencedSerie {
    pub series_instance_uid: String,
    pub contour_image_sequence: Vec<ContourImage>,
}

#[derive(Clone, Debug, Default, )]
pub struct ContourImage {
    pub sops: Vec<Sop>,
}

#[derive(Clone, Debug, Default, )]
pub struct StructureSetROI {
    pub roi_number: i32,
    pub referenced_frame_of_reference_uid: String,
    pub roi_name: String,
    pub roi_generation_algorithm: String,
    pub roi_generation_description: String,
    pub roi_volume: f64,
}

#[derive(Clone, Debug, Default, )]
pub struct RoiContour {
    pub roi_display_color: String,
    pub contour_sequence: Vec<Contour>,
}

#[derive(Clone, Debug, Default, )]
pub struct Contour {
    pub contour_number: i32,
    pub contour_image_sequence: Vec<ContourImage>,
    pub contour_geometry_type: String,
    pub number_of_contour_points: i32,
    pub contour_data: Vec<f64>,
}

#[derive(Clone, Debug, Default, )]
pub struct RTRoiObservation {
    pub observation_number: i32,
    pub referenced_roi_number: i32,
    pub roi_observation_label: String,
    pub rt_roi_interpreted_type: String,
    pub roi_interpreter: String,
}