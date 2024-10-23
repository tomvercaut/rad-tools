use std::path::PathBuf;
use std::time::SystemTime;

use dicom_dictionary_std::uids::{
    CT_IMAGE_STORAGE, ENHANCED_CT_IMAGE_STORAGE, MR_IMAGE_STORAGE,
    POSITRON_EMISSION_TOMOGRAPHY_IMAGE_STORAGE,
};

#[derive(Clone, Debug)]
pub enum DicomFile {
    None,
    Image(Image),
    RTStruct(RTStruct),
    RTPlan(RTPlan),
    RTDose(RTDose),
}

impl Default for DicomFile {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FileInfo {
    pub path: PathBuf,
    pub last_modified: Option<SystemTime>,
}

pub trait HasModality {
    fn modality(&self) -> Modality;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Modality {
    None,
    Ct,
    EnhancedCt,
    Mr,
    Pt,
    RtStruct,
    RtPlan,
    RtDose,
    Other,
}
impl Default for Modality {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug, Default)]
pub struct SopClass {
    pub class_uid: String,
    pub instance_uid: String,
}

#[derive(Clone, Debug, Default)]
pub struct ReferencedSopClass {
    pub ref_class_uid: String,
    pub ref_instance_uid: String,
}

#[derive(Clone, Debug, Default)]
pub struct ReferencedFrameOfReference {
    pub frame_of_reference_uid: String,
    pub rt_referenced_study_sequence: Vec<RTReferencedStudy>,
}

#[derive(Clone, Debug, Default)]
pub struct RTReferencedStudy {
    pub referenced_sop: ReferencedSopClass,
    pub rt_referenced_series: Vec<RTReferencedSerie>,
}

#[derive(Clone, Debug, Default)]
pub struct RTReferencedSerie {
    pub instance_uid: String,
    pub contour_image_sequence: Vec<ReferencedSopClass>,
}

#[derive(Clone, Debug, Default)]
pub struct Image {
    pub file_info: FileInfo,
    pub patient_id: String,
    pub patient_name: String,
    pub sop: SopClass,
    pub study_instance_uid: String,
    pub series_instance_uid: String,
    pub frame_of_reference_uid: String,
}

impl Image {
    pub fn is_ct(&self) -> bool {
        self.sop.class_uid == CT_IMAGE_STORAGE
    }

    pub fn is_enhanced_ct(&self) -> bool {
        self.sop.class_uid == ENHANCED_CT_IMAGE_STORAGE
    }

    pub fn is_mr(&self) -> bool {
        self.sop.class_uid == MR_IMAGE_STORAGE
    }

    pub fn is_pt(&self) -> bool {
        self.sop.class_uid == POSITRON_EMISSION_TOMOGRAPHY_IMAGE_STORAGE
    }
}

impl HasModality for Image {
    fn modality(&self) -> Modality {
        if self.is_ct() {
            Modality::Ct
        } else if self.is_enhanced_ct() {
            Modality::EnhancedCt
        } else if self.is_mr() {
            Modality::Mr
        } else if self.is_pt() {
            Modality::Pt
        } else {
            Modality::Other
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct RTStruct {
    pub file_info: FileInfo,
    pub patient_id: String,
    pub sop: SopClass,
    pub study_instance_uid: String,
    pub series_instance_uid: String,
    pub label: String,
    pub referenced_frame_of_references: Vec<ReferencedFrameOfReference>,
}

#[derive(Clone, Debug, Default)]
pub struct RTPlan {
    pub file_info: FileInfo,
    pub patient_id: String,
    pub patient_name: String,
    pub sop: SopClass,
    pub plan_name: String,
    pub plan_label: String,
    pub referenced_structure_sets: Vec<ReferencedSopClass>,
}

impl HasModality for RTPlan {
    fn modality(&self) -> Modality {
        Modality::RtPlan
    }
}

#[derive(Clone, Debug, Default)]
pub struct RTDose {
    pub file_info: FileInfo,
    pub patient_id: String,
    pub sop: SopClass,
    pub referenced_rtplan_sequence: Vec<ReferencedSopClass>,
}

impl HasModality for RTDose {
    fn modality(&self) -> Modality {
        Modality::RtDose
    }
}
