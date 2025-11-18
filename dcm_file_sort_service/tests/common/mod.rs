use dicom_core::VR;
use dicom_dictionary_std::tags::{
    MODALITY, PATIENT_BIRTH_DATE, PATIENT_ID, SOP_CLASS_UID, SOP_INSTANCE_UID,
};
use dicom_dictionary_std::uids::{
    CT_IMAGE_STORAGE, RT_DOSE_STORAGE, RT_PLAN_STORAGE, RT_STRUCTURE_SET_STORAGE,
};
use dicom_object::{FileMetaTableBuilder, InMemDicomObject};
use rad_tools_dcm_file_sort_service::path_gen::DicomPathGeneratorType;
use std::env::temp_dir;
use std::fmt::Display;
use std::path::Path;
use tracing::debug;

pub fn init_logger(level: tracing::Level) {
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_max_level(level)
        .init();
}

pub fn test_dir(test_name: &str) -> std::path::PathBuf {
    temp_dir().join("dcm_file_sort_service").join(test_name)
}

fn meta_table_ct() -> FileMetaTableBuilder {
    FileMetaTableBuilder::new()
        .transfer_syntax(dicom_transfer_syntax_registry::default().erased().uid())
        .media_storage_sop_class_uid(CT_IMAGE_STORAGE)
}

fn meta_table_ss() -> FileMetaTableBuilder {
    FileMetaTableBuilder::new()
        .transfer_syntax(dicom_transfer_syntax_registry::default().erased().uid())
        .media_storage_sop_class_uid(RT_STRUCTURE_SET_STORAGE)
}

fn meta_table_plan() -> FileMetaTableBuilder {
    FileMetaTableBuilder::new()
        .transfer_syntax(dicom_transfer_syntax_registry::default().erased().uid())
        .media_storage_sop_class_uid(RT_PLAN_STORAGE)
}

fn meta_table_dose() -> FileMetaTableBuilder {
    FileMetaTableBuilder::new()
        .transfer_syntax(dicom_transfer_syntax_registry::default().erased().uid())
        .media_storage_sop_class_uid(RT_DOSE_STORAGE)
}

#[derive(Debug, Clone)]
pub struct TestData {
    pub sop_instance_uid: String,
    pub patient_id: String,
    pub date_of_birth: Option<String>,
    pub modality: String,
    pub path: std::path::PathBuf,
    pub meta_table: FileMetaTableBuilder,
    pub result_path: std::path::PathBuf,
}

impl Display for TestData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TestData {{ sop_instance_uid: {:?}, patient_id: {:?}, date_of_birth: {:?}, modality: {:?}, path: {:#?}, result_path: {:#?} }}",
            self.sop_instance_uid,
            self.patient_id,
            self.date_of_birth,
            self.modality,
            self.path,
            self.result_path,
        )
    }
}

pub fn dcm_file_sort_app() -> std::ffi::OsString {
    assert_cmd::Command::cargo_bin("dcm_file_sort")
        .unwrap()
        .get_program()
        .to_os_string()
}

pub fn create_test_data<P>(
    p: P,
    odir: P,
    create_cts: bool,
    create_ss: bool,
    create_plan: bool,
    create_dose: bool,
    generator_type: DicomPathGeneratorType,
) -> Vec<TestData>
where
    P: AsRef<Path>,
{
    let p = p.as_ref();
    let odir = odir.as_ref();
    let mut v = vec![];

    for j in 0..3 {
        let dob = Some(format!("19900{}0{}", j + 1, j + 1));
        let patient_id = format!("900{}0{}", j + 1, j + 1);
        let ct_sop_instance_uid = format!("1.2.3.4.5.{}", j);
        let ss_sop_instance_uid = format!("2.2.3.4.5.{}", j);
        let plan_sop_instance_uid = format!("3.2.3.4.5.{}", j);
        let dose_sop_instance_uid = format!("4.2.3.4.5.{}", j);
        if create_cts {
            for i in 0..5 {
                let result_path = if i == 0 {
                    match generator_type {
                        DicomPathGeneratorType::Default => odir
                            .join("patient_id")
                            .join(&patient_id)
                            .join(format!("CT.{}.dcm", &ct_sop_instance_uid)),
                        DicomPathGeneratorType::Uzg => odir
                            .join(format!("0{}0{}", j + 1, j + 1))
                            .join(&patient_id)
                            .join(format!("CT.{}.dcm", &ct_sop_instance_uid)),
                    }
                } else {
                    match generator_type {
                        DicomPathGeneratorType::Default => odir
                            .join("patient_id")
                            .join(&patient_id)
                            .join(format!("CT.{}_{}.dcm", &ct_sop_instance_uid, i - 1)),
                        DicomPathGeneratorType::Uzg => odir
                            .join(format!("0{}0{}", j + 1, j + 1))
                            .join(&patient_id)
                            .join(format!("CT.{}_{}.dcm", &ct_sop_instance_uid, i - 1)),
                    }
                };
                v.push(TestData {
                    sop_instance_uid: ct_sop_instance_uid.clone(),
                    patient_id: patient_id.clone(),
                    date_of_birth: dob.clone(),
                    modality: "CT".to_string(),
                    path: p.join(format!("CT_{}_{}.dcm", j, i)),
                    meta_table: meta_table_ct(),
                    result_path,
                });
            }
        }
        if create_ss {
            for i in 0..3 {
                let result_path = if i == 0 {
                    match generator_type {
                        DicomPathGeneratorType::Default => odir
                            .join("patient_id")
                            .join(&patient_id)
                            .join(format!("RTSTRUCT.{}.dcm", &ss_sop_instance_uid)),
                        DicomPathGeneratorType::Uzg => odir
                            .join(format!("0{}0{}", j + 1, j + 1))
                            .join(&patient_id)
                            .join(format!("RTSTRUCT.{}.dcm", &ss_sop_instance_uid)),
                    }
                } else {
                    match generator_type {
                        DicomPathGeneratorType::Default => odir
                            .join("patient_id")
                            .join(&patient_id)
                            .join(format!("RTSTRUCT.{}_{}.dcm", &ss_sop_instance_uid, i - 1)),
                        DicomPathGeneratorType::Uzg => odir
                            .join(format!("0{}0{}", j + 1, j + 1))
                            .join(&patient_id)
                            .join(format!("RTSTRUCT.{}_{}.dcm", &ss_sop_instance_uid, i - 1)),
                    }
                };
                v.push(TestData {
                    sop_instance_uid: ss_sop_instance_uid.clone(),
                    patient_id: patient_id.clone(),
                    date_of_birth: dob.clone(),
                    modality: "RTSTRUCT".to_string(),
                    path: p.join(format!("RTSTRUCT_{}_{}.dcm", j, i - 1)),
                    meta_table: meta_table_ss(),
                    result_path,
                });
            }
        }
        if create_plan {
            for i in 0..2 {
                let result_path = if i == 0 {
                    match generator_type {
                        DicomPathGeneratorType::Default => odir
                            .join("patient_id")
                            .join(&patient_id)
                            .join(format!("RTPLAN.{}.dcm", &plan_sop_instance_uid)),
                        DicomPathGeneratorType::Uzg => odir
                            .join(format!("0{}0{}", j + 1, j + 1))
                            .join(&patient_id)
                            .join(format!("RTPLAN.{}.dcm", &plan_sop_instance_uid)),
                    }
                } else {
                    match generator_type {
                        DicomPathGeneratorType::Default => odir
                            .join("patient_id")
                            .join(&patient_id)
                            .join(format!("RTPLAN.{}_{}.dcm", &plan_sop_instance_uid, i - 1)),
                        DicomPathGeneratorType::Uzg => odir
                            .join(format!("0{}0{}", j + 1, j + 1))
                            .join(&patient_id)
                            .join(format!("RTPLAN.{}_{}.dcm", &plan_sop_instance_uid, i - 1)),
                    }
                };
                v.push(TestData {
                    sop_instance_uid: plan_sop_instance_uid.clone(),
                    patient_id: patient_id.clone(),
                    date_of_birth: dob.clone(),
                    modality: "RTPLAN".to_string(),
                    path: p.join(format!("RTPLAN_{}_{}.dcm", j, i - 1)),
                    meta_table: meta_table_plan(),
                    result_path,
                });
            }
        }
        if create_dose {
            for i in 0..3 {
                let result_path = if i == 0 {
                    match generator_type {
                        DicomPathGeneratorType::Default => odir
                            .join("patient_id")
                            .join(&patient_id)
                            .join(format!("RTDOSE.{}.dcm", &dose_sop_instance_uid)),
                        DicomPathGeneratorType::Uzg => odir
                            .join(format!("0{}0{}", j + 1, j + 1))
                            .join(&patient_id)
                            .join(format!("RTDOSE.{}.dcm", &dose_sop_instance_uid)),
                    }
                } else {
                    match generator_type {
                        DicomPathGeneratorType::Default => odir
                            .join("patient_id")
                            .join(&patient_id)
                            .join(format!("RTDOSE.{}_{}.dcm", &dose_sop_instance_uid, i - 1)),
                        DicomPathGeneratorType::Uzg => odir
                            .join(format!("0{}0{}", j + 1, j + 1))
                            .join(&patient_id)
                            .join(format!("RTDOSE.{}_{}.dcm", &dose_sop_instance_uid, i - 1)),
                    }
                };
                v.push(TestData {
                    sop_instance_uid: dose_sop_instance_uid.clone(),
                    patient_id: patient_id.clone(),
                    date_of_birth: None,
                    modality: "RTDOSE".to_string(),
                    path: p.join(format!("RTDOSE_{}_{}.dcm", j, i)),
                    meta_table: meta_table_dose(),
                    result_path,
                });
            }
        }
    }

    for td in &v {
        let mut obj = InMemDicomObject::new_empty();
        obj.put_str(SOP_INSTANCE_UID, VR::UI, td.sop_instance_uid.as_str());
        obj.put_str(PATIENT_ID, VR::LO, td.patient_id.as_str());
        if let Some(dob) = td.date_of_birth.as_ref() {
            obj.put_str(PATIENT_BIRTH_DATE, VR::DA, dob);
        }
        obj.put_str(MODALITY, VR::CS, td.modality.as_str());
        match td.modality.as_str() {
            "CT" => {
                obj.put_str(SOP_CLASS_UID, VR::UI, CT_IMAGE_STORAGE);
            }
            "RTSTRUCT" => {
                obj.put_str(SOP_CLASS_UID, VR::UI, RT_STRUCTURE_SET_STORAGE);
            }
            "RTPLAN" => {
                obj.put_str(SOP_CLASS_UID, VR::UI, RT_PLAN_STORAGE);
            }
            "RTDOSE" => {
                obj.put_str(SOP_CLASS_UID, VR::UI, RT_DOSE_STORAGE);
            }
            _ => {
                panic!("Unsupported DICOM modality in test data: {}", td.modality);
            }
        }
        let file_obj = obj.with_meta(td.meta_table.clone()).unwrap();
        file_obj.write_to_file(&td.path).unwrap();
    }
    v
}

pub fn has_no_files<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let entries = std::fs::read_dir(path)
        .expect(&format!("Failed to read directory: {:#?}", path))
        .filter(|entry| {
            let is_ok = entry.is_ok();
            if !is_ok {
                return false;
            }
            let entry = entry.as_ref().unwrap();
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            if file_name == "." || file_name == ".." {
                return false;
            }
            true
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to collect directory entries");
    debug!("Found {} files in directory: {:#?}", entries.len(), path);
    entries.is_empty()
}
