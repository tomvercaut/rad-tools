use crate::{ApprovalStatus, PersonName, Sop};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Clone, Debug, Default)]
pub struct RTStruct {
    pub specific_character_set: String,
    pub instance_creation_dt: Option<NaiveDateTime>,
    pub sop: Sop,
    pub study_dt: Option<NaiveDateTime>,
    pub accession_number: Option<String>,
    pub manufacturer: Option<String>,
    pub referring_physician_name: Option<PersonName>,
    pub manufacturer_model_name: Option<String>,
    pub patient_name: PersonName,
    pub patient_id: String,
    pub patient_birth_date: Option<NaiveDate>,
    pub patient_sex: String,
    pub patient_identity_removed: bool,
    pub deidentification_method: Option<String>,
    pub software_versions: Option<String>,
    pub study_instance_uid: String,
    pub series_instance_uid: String,
    pub study_id: Option<String>,
    pub series_number: i32,
    pub instance_number: i32,
    pub frame_of_reference_uid: String,
    pub position_reference_indicator: Option<String>,
    pub structure_set_label: String,
    pub structure_set_dt: Option<NaiveDateTime>,
    pub referenced_frame_of_reference_seq: Vec<ReferencedFrameOfReference>,
    pub structure_set_roi_sequence: Vec<StructureSetROI>,
    pub roi_contour_sequence: Vec<RoiContour>,
    pub rt_roi_observations_sequence: Vec<RTRoiObservation>,
    pub approval_status: Option<ApprovalStatus>,
}

#[derive(Clone, Debug, Default)]
pub struct ReferencedFrameOfReference {
    pub frame_of_reference_uid: String,
    pub rt_referenced_study_sequence: Vec<RTReferencedStudy>,
}

#[derive(Clone, Debug, Default)]
pub struct RTReferencedStudy {
    pub referenced_sop: Sop,
    pub rt_referenced_series_sequence: Vec<RTReferencedSerie>,
}

#[derive(Clone, Debug, Default)]
pub struct RTReferencedSerie {
    pub series_instance_uid: String,
    pub contour_image_sequence: Vec<Sop>,
}

#[derive(Clone, Debug, Default)]
pub struct StructureSetROI {
    pub roi_number: i32,
    pub referenced_frame_of_reference_uid: String,
    pub roi_name: Option<String>,
    pub roi_generation_algorithm: Option<String>,
    pub roi_generation_description: Option<String>,
    pub roi_volume: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct RoiContour {
    pub roi_display_color: Option<[u8; 3]>,
    pub contour_sequence: Option<Vec<Contour>>,
    pub referenced_roi_number: i32,
}

#[derive(Clone, Debug, Default)]
pub struct Contour {
    pub contour_number: Option<i32>,
    pub contour_image_sequence: Option<Vec<Sop>>,
    pub contour_geometry_type: String,
    pub number_of_contour_points: i32,
    pub contour_data: Vec<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct RTRoiObservation {
    pub observation_number: i32,
    pub referenced_roi_number: i32,
    pub rt_roi_interpreted_type: Option<String>,
    pub roi_interpreter: Option<String>,
}
