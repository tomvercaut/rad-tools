use crate::Error;
use crate::pattern::{SearchPatterns, Selector};
use dicom_core::value::Value;
use dicom_object::InMemDicomObject;
use std::str::FromStr;
use tracing::error;

/// Searches for DICOM elements in the provided DICOM object that match the given search pattern.
///
/// # Arguments
/// * `obj` - The DICOM object to search through
/// * `pattern` - A string representing the search pattern in the format "tag[selector]/tag[selector]/..."
///
/// # Returns
/// * `Ok(Vec<GrepResult>)` - A vector of matching elements with their paths and values
/// * `Err(Error)` - If the pattern is invalid
pub fn grep(obj: &InMemDicomObject, pattern: &str) -> Result<Vec<GrepResult>, Error> {
    let patterns = SearchPatterns::from_str(pattern).map_err(|e| {
        error!("Unable to create search patterns: {e:#?}");
        Error::NoMatchingElementFound
    })?;

    let v = grep_matching_elements(obj, &patterns, 0);
    Ok(v)
}

/// Represents a matched DICOM element from a grep search operation
#[derive(Debug)]
pub struct GrepResult {
    /// Path of the element value in the DICOM object.
    /// Example: (1234,5678)[2]/(9876,5432)
    pub path: String,
    /// String representation of the element's value
    pub value: String,
}

/// Recursively searches for DICOM elements that match the given patterns
///
/// # Arguments
/// * `obj` - The DICOM object to search through
/// * `patterns` - The parsed search patterns to match against
/// * `ipattern` - The current pattern index being processed
///
/// # Returns
/// A vector of GrepResult containing matching elements and their paths
fn grep_matching_elements(
    obj: &InMemDicomObject,
    patterns: &SearchPatterns,
    ipattern: usize,
) -> Vec<GrepResult> {
    let pattern = &patterns.patterns[ipattern];
    let mut vec = vec![];
    let stag = format!("{}", pattern.tag);
    if let Ok(element) = obj.element(pattern.tag) {
        match element.value() {
            Value::Primitive(primitive) => {
                vec.push(GrepResult {
                    path: stag,
                    value: primitive.to_string(),
                });
            }
            Value::Sequence(seq) => {
                let items = seq.items();

                if pattern.selectors.is_empty() {
                    vec.push(GrepResult {
                        path: stag,
                        value: "Sequence".to_string(),
                    });
                } else {
                    for selector in &pattern.selectors {
                        match selector {
                            Selector::All => {
                                let nested_results = items
                                    .iter()
                                    .map(|item| {
                                        grep_matching_elements(item, patterns, ipattern + 1)
                                    })
                                    .collect::<Vec<_>>();
                                for (index, nested_result) in nested_results.into_iter().enumerate()
                                {
                                    let path_prefix = format!("{stag}[{index}]");
                                    append_result(&mut vec, path_prefix, nested_result);
                                }
                            }
                            Selector::Index(index) => {
                                if let Some(sub_item) = items.get(*index) {
                                    let nested_result =
                                        grep_matching_elements(sub_item, patterns, ipattern + 1);
                                    let path_prefix = format!("{stag}[{index}]");
                                    append_result(&mut vec, path_prefix, nested_result);
                                }
                            }
                            Selector::Range(start, end) => {
                                for index in *start..*end {
                                    if let Some(sub_item) = items.get(index) {
                                        let path_prefix = format!("{stag}[{index}]");
                                        let nested_result = grep_matching_elements(
                                            sub_item,
                                            patterns,
                                            ipattern + 1,
                                        );
                                        append_result(&mut vec, path_prefix, nested_result);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Value::PixelSequence(_) => {
                vec.push(GrepResult {
                    path: stag,
                    value: "PixelSequence".to_string(),
                });
            }
        }
    }
    vec
}

/// Appends nested search results to the main results vector with updated paths
///
/// # Arguments
/// * `vec` - The vector to append results to
/// * `prefix` - The path prefix to prepend to nested results
/// * `results` - The nested results to append
fn append_result(vec: &mut Vec<GrepResult>, prefix: String, results: Vec<GrepResult>) {
    for fr in results {
        let nfr = GrepResult {
            path: format!("{}/{}", &prefix, fr.path),
            value: fr.value,
        };
        vec.push(nfr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dicom_core::value::DataSetSequence;
    use dicom_core::{DataElement, VR};
    use dicom_dictionary_std::tags::{
        CODE_MEANING, CODE_VALUE, CODING_SCHEME_DESIGNATOR, CTDI_PHANTOM_TYPE_CODE_SEQUENCE,
        PATIENT_ID, PATIENT_NAME, REFERENCED_SOP_CLASS_UID, REFERENCED_SOP_INSTANCE_UID,
        REFERENCED_STUDY_SEQUENCE, REQUEST_ATTRIBUTES_SEQUENCE, SERIES_DATE, SERIES_TIME,
        SOP_CLASS_UID, STUDY_DATE, STUDY_TIME,
    };
    use dicom_dictionary_std::uids::CT_IMAGE_STORAGE;
    use dicom_object::FileMetaTableBuilder;

    fn create_dicom_model() -> InMemDicomObject {
        let mut obj = InMemDicomObject::new_empty();
        obj.put_str(SOP_CLASS_UID, VR::UI, CT_IMAGE_STORAGE);
        obj.put_str(PATIENT_ID, VR::LO, "123456");
        obj.put_str(PATIENT_NAME, VR::PN, "Last^First");
        obj.put_str(STUDY_DATE, VR::DA, "20230102");
        obj.put_str(STUDY_TIME, VR::TM, "171657.858653");
        obj.put_str(SERIES_DATE, VR::DA, "20230102");
        obj.put_str(SERIES_TIME, VR::TM, "173657.158653");

        // Sequence on the root level
        let mut code_item1 = InMemDicomObject::new_empty();
        code_item1.put_str(CODE_VALUE, VR::SH, "123456");
        code_item1.put_str(CODING_SCHEME_DESIGNATOR, VR::SH, "DCM1");
        code_item1.put_str(CODE_MEANING, VR::LO, "Coding Meaning1");

        let mut code_item2 = InMemDicomObject::new_empty();
        code_item2.put_str(CODE_VALUE, VR::SH, "654321");
        code_item2.put_str(CODING_SCHEME_DESIGNATOR, VR::SH, "DCM2");
        code_item2.put_str(CODE_MEANING, VR::LO, "Coding Meaning2");

        let ctdi_phantom_type_code_sequence = DataSetSequence::from(vec![code_item1, code_item2]);
        obj.put(DataElement::new(
            CTDI_PHANTOM_TYPE_CODE_SEQUENCE,
            VR::SQ,
            ctdi_phantom_type_code_sequence,
        ));

        // Sequence nested in another sequence
        {
            let mut study_item1 = InMemDicomObject::new_empty();
            study_item1.put_str(REFERENCED_SOP_CLASS_UID, VR::UI, "RefSopClass1");
            study_item1.put_str(REFERENCED_SOP_INSTANCE_UID, VR::UI, "RefSopInstance1");

            let mut study_item2 = InMemDicomObject::new_empty();
            study_item2.put_str(REFERENCED_SOP_CLASS_UID, VR::UI, "RefSopClass2");
            study_item2.put_str(REFERENCED_SOP_INSTANCE_UID, VR::UI, "RefSopInstance2");

            let mut study_item3 = InMemDicomObject::new_empty();
            study_item3.put_str(REFERENCED_SOP_CLASS_UID, VR::UI, "RefSopClass3");
            study_item3.put_str(REFERENCED_SOP_INSTANCE_UID, VR::UI, "RefSopInstance3");

            let ref_study_seq1 = DataSetSequence::from(vec![study_item1]);
            let ref_study_seq2 = DataSetSequence::from(vec![study_item2, study_item3]);

            let mut ref_study_seq1_item = InMemDicomObject::new_empty();
            ref_study_seq1_item.put(DataElement::new(
                REFERENCED_STUDY_SEQUENCE,
                VR::SQ,
                ref_study_seq1,
            ));
            let mut ref_study_seq2_item = InMemDicomObject::new_empty();
            ref_study_seq2_item.put(DataElement::new(
                REFERENCED_STUDY_SEQUENCE,
                VR::SQ,
                ref_study_seq2,
            ));

            let req_attr_seq =
                DataSetSequence::from(vec![ref_study_seq1_item, ref_study_seq2_item]);
            obj.put(DataElement::new(
                REQUEST_ATTRIBUTES_SEQUENCE,
                VR::SQ,
                req_attr_seq,
            ));
        }

        obj
    }

    #[allow(dead_code)]
    fn write_tmp_file(obj: &InMemDicomObject) {
        // Write a temporary DICOM file
        let temp_dir = std::env::temp_dir();
        let tdir = temp_dir.join("rad_tools_dcm_grep");
        if !tdir.is_dir() {
            std::fs::create_dir_all(&tdir).unwrap();
        }
        let file_obj = obj
            .clone()
            .with_meta(
                FileMetaTableBuilder::new()
                    .transfer_syntax(dicom_transfer_syntax_registry::default().erased().uid())
                    .media_storage_sop_class_uid(CT_IMAGE_STORAGE),
            )
            .unwrap();
        let tmp_input = tdir.join("rad_tools_dcm_grep.dcm");
        println!("Writing a temporary file to: {:?}", tmp_input);
        file_obj.write_to_file(tmp_input.as_path()).unwrap();
        assert!(tmp_input.is_file());
    }

    #[test]
    fn find_non_nested_elements() {
        let obj = create_dicom_model();
        match grep(&obj, &SOP_CLASS_UID.to_string()) {
            Ok(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].path, SOP_CLASS_UID.to_string());
                assert_eq!(results[0].value, CT_IMAGE_STORAGE);
            }
            Err(e) => panic!("Unable to find SOP_CLASS_UID: {e:#?}"),
        }
        match grep(&obj, &PATIENT_ID.to_string()) {
            Ok(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].path, PATIENT_ID.to_string());
                assert_eq!(results[0].value, "123456");
            }
            Err(e) => panic!("Unable to find PATIENT_ID: {e:#?}"),
        }
        match grep(&obj, &PATIENT_NAME.to_string()) {
            Ok(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].path, PATIENT_NAME.to_string());
                assert_eq!(results[0].value, "Last^First");
            }
            Err(e) => panic!("Unable to find PATIENT_NAME: {e:#?}"),
        }
        match grep(&obj, &STUDY_DATE.to_string()) {
            Ok(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].path, STUDY_DATE.to_string());
                assert_eq!(results[0].value, "20230102");
            }
            Err(e) => panic!("Unable to find STUDY_DATE: {e:#?}"),
        }
        match grep(&obj, &STUDY_TIME.to_string()) {
            Ok(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].path, STUDY_TIME.to_string());
                assert_eq!(results[0].value, "171657.858653");
            }
            Err(e) => panic!("Unable to find STUDY_TIME: {e:#?}"),
        }
        match grep(&obj, &SERIES_DATE.to_string()) {
            Ok(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].path, SERIES_DATE.to_string());
                assert_eq!(results[0].value, "20230102");
            }
            Err(e) => panic!("Unable to find SERIES_DATE: {e:#?}"),
        }
        match grep(&obj, &SERIES_TIME.to_string()) {
            Ok(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].path, SERIES_TIME.to_string());
                assert_eq!(results[0].value, "173657.158653");
            }
            Err(e) => panic!("Unable to find SERIES_TIME: {e:#?}"),
        }
    }

    #[test]
    fn find_sequence_level1() {
        let obj = create_dicom_model();
        match grep(&obj, &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string()) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(result[0].path, CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string());
                assert_eq!(result[0].value, "Sequence");
            }
            Err(e) => {
                panic!("Unable to find CTDI_PHANTOM_TYPE_CODE_SEQUENCE: {e:#?}");
            }
        }
    }
    #[test]
    fn find_sequence_level2() {
        let obj = create_dicom_model();
        match grep(
            &obj,
            &format!(
                "{}[*]/{}",
                REQUEST_ATTRIBUTES_SEQUENCE, REFERENCED_STUDY_SEQUENCE
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 2);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[0]/{}",
                        REQUEST_ATTRIBUTES_SEQUENCE, REFERENCED_STUDY_SEQUENCE
                    )
                );
                assert_eq!(result[0].value, "Sequence");
                assert_eq!(
                    result[1].path,
                    format!(
                        "{}[1]/{}",
                        REQUEST_ATTRIBUTES_SEQUENCE, REFERENCED_STUDY_SEQUENCE
                    )
                );
                assert_eq!(result[1].value, "Sequence");
            }
            Err(e) => {
                panic!("Unable to find REFERENCED_STUDY_SEQUENCE: {e:#?}");
            }
        }
    }

    #[test]
    fn find_all_nested_elements_level1() {
        let obj = create_dicom_model();
        match grep(
            &obj,
            &format!(
                "{}[*]/{}",
                &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                &CODE_VALUE.to_string()
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 2);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[0]/{}",
                        &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                        &CODE_VALUE.to_string()
                    )
                );
                assert_eq!(result[0].value, "123456");
                assert_eq!(
                    result[1].path,
                    format!(
                        "{}[1]/{}",
                        &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                        &CODE_VALUE.to_string()
                    )
                );
                assert_eq!(result[1].value, "654321");
            }
            Err(e) => {
                panic!("Unable to find all CTDI_PHANTOM_TYPE_CODE_SEQUENCE CodeValues: {e:#?}");
            }
        }
        match grep(
            &obj,
            &format!(
                "{}[*]/{}",
                &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                &CODING_SCHEME_DESIGNATOR.to_string()
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 2);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[0]/{}",
                        &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                        &CODING_SCHEME_DESIGNATOR.to_string()
                    )
                );
                assert_eq!(result[0].value, "DCM1");
                assert_eq!(
                    result[1].path,
                    format!(
                        "{}[1]/{}",
                        &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                        &CODING_SCHEME_DESIGNATOR.to_string()
                    )
                );
                assert_eq!(result[1].value, "DCM2");
            }
            Err(e) => {
                panic!(
                    "Unable to find all CTDI_PHANTOM_TYPE_CODE_SEQUENCE CodingSchemeDesignators: {e:#?}"
                );
            }
        }
        match grep(
            &obj,
            &format!(
                "{}[*]/{}",
                &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                &CODE_MEANING.to_string()
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 2);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[0]/{}",
                        &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                        &CODE_MEANING.to_string()
                    )
                );
                assert_eq!(result[0].value, "Coding Meaning1");
                assert_eq!(
                    result[1].path,
                    format!(
                        "{}[1]/{}",
                        &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                        &CODE_MEANING.to_string()
                    )
                );
                assert_eq!(result[1].value, "Coding Meaning2");
            }
            Err(e) => {
                panic!("Unable to find all CTDI_PHANTOM_TYPE_CODE_SEQUENCE CodeMeanings: {e:#?}");
            }
        }
    }

    #[test]
    fn find_nested_elements_level1_by_index() {
        let obj = create_dicom_model();
        match grep(
            &obj,
            &format!(
                "{}[0]/{}",
                &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                &CODE_VALUE.to_string()
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[0]/{}",
                        &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                        &CODE_VALUE.to_string()
                    )
                );
                assert_eq!(result[0].value, "123456");
            }
            Err(e) => {
                panic!(
                    "Unable to find the first CTDI_PHANTOM_TYPE_CODE_SEQUENCE CodeValue: {e:#?}"
                );
            }
        }
        match grep(
            &obj,
            &format!(
                "{}[1]/{}",
                &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                &CODE_VALUE.to_string()
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[1]/{}",
                        &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                        &CODE_VALUE.to_string()
                    )
                );
                assert_eq!(result[0].value, "654321");
            }
            Err(e) => {
                panic!(
                    "Unable to find the second CTDI_PHANTOM_TYPE_CODE_SEQUENCE CodeValue: {e:#?}"
                );
            }
        }
        match grep(
            &obj,
            &format!(
                "{}[0]/{}",
                &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                &CODING_SCHEME_DESIGNATOR.to_string()
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[0]/{}",
                        &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                        &CODING_SCHEME_DESIGNATOR.to_string()
                    )
                );
                assert_eq!(result[0].value, "DCM1");
            }
            Err(e) => {
                panic!(
                    "Unable to find the first CTDI_PHANTOM_TYPE_CODE_SEQUENCE CodingSchemeDesignator: {e:#?}"
                );
            }
        }
        match grep(
            &obj,
            &format!(
                "{}[1]/{}",
                &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                &CODING_SCHEME_DESIGNATOR.to_string()
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[1]/{}",
                        &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                        &CODING_SCHEME_DESIGNATOR.to_string()
                    )
                );
                assert_eq!(result[0].value, "DCM2");
            }
            Err(e) => {
                panic!(
                    "Unable to find the second CTDI_PHANTOM_TYPE_CODE_SEQUENCE CodingSchemeDesignator: {e:#?}"
                );
            }
        }
        match grep(
            &obj,
            &format!(
                "{}[0]/{}",
                &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                &CODE_MEANING.to_string()
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[0]/{}",
                        &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                        &CODE_MEANING.to_string()
                    )
                );
                assert_eq!(result[0].value, "Coding Meaning1");
            }
            Err(e) => {
                panic!(
                    "Unable to find the first CTDI_PHANTOM_TYPE_CODE_SEQUENCE CodeMeaning: {e:#?}"
                );
            }
        }
        match grep(
            &obj,
            &format!(
                "{}[1]/{}",
                &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                &CODE_MEANING.to_string()
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[1]/{}",
                        &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(),
                        &CODE_MEANING.to_string()
                    )
                );
                assert_eq!(result[0].value, "Coding Meaning2");
            }
            Err(e) => {
                panic!(
                    "Unable to find the second CTDI_PHANTOM_TYPE_CODE_SEQUENCE CodeMeaning: {e:#?}"
                );
            }
        }
    }

    #[test]
    fn find_all_nested_elements_level2() {
        let obj = create_dicom_model();
        match grep(
            &obj,
            &format!(
                "{}[*]/{}[*]/{}",
                &REQUEST_ATTRIBUTES_SEQUENCE, &REFERENCED_STUDY_SEQUENCE, &REFERENCED_SOP_CLASS_UID
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 3);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[0]/{}[0]/{}",
                        &REQUEST_ATTRIBUTES_SEQUENCE,
                        &REFERENCED_STUDY_SEQUENCE,
                        &REFERENCED_SOP_CLASS_UID,
                    )
                );
                assert_eq!(result[0].value, "RefSopClass1");
                assert_eq!(
                    result[1].path,
                    format!(
                        "{}[1]/{}[0]/{}",
                        &REQUEST_ATTRIBUTES_SEQUENCE,
                        &REFERENCED_STUDY_SEQUENCE,
                        &REFERENCED_SOP_CLASS_UID,
                    )
                );
                assert_eq!(result[1].value, "RefSopClass2");
                assert_eq!(
                    result[2].path,
                    format!(
                        "{}[1]/{}[1]/{}",
                        &REQUEST_ATTRIBUTES_SEQUENCE,
                        &REFERENCED_STUDY_SEQUENCE,
                        &REFERENCED_SOP_CLASS_UID,
                    )
                );
                assert_eq!(result[2].value, "RefSopClass3");
            }
            Err(e) => {
                panic!("Unable to find all ReferencedSopClassUIDs: {e:#?}");
            }
        }

        match grep(
            &obj,
            &format!(
                "{}[*]/{}[*]/{}",
                &REQUEST_ATTRIBUTES_SEQUENCE,
                &REFERENCED_STUDY_SEQUENCE,
                &REFERENCED_SOP_INSTANCE_UID
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 3);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[0]/{}[0]/{}",
                        &REQUEST_ATTRIBUTES_SEQUENCE,
                        &REFERENCED_STUDY_SEQUENCE,
                        &REFERENCED_SOP_INSTANCE_UID,
                    )
                );
                assert_eq!(result[0].value, "RefSopInstance1");
                assert_eq!(
                    result[1].path,
                    format!(
                        "{}[1]/{}[0]/{}",
                        &REQUEST_ATTRIBUTES_SEQUENCE,
                        &REFERENCED_STUDY_SEQUENCE,
                        &REFERENCED_SOP_INSTANCE_UID,
                    )
                );
                assert_eq!(result[1].value, "RefSopInstance2");
                assert_eq!(
                    result[2].path,
                    format!(
                        "{}[1]/{}[1]/{}",
                        &REQUEST_ATTRIBUTES_SEQUENCE,
                        &REFERENCED_STUDY_SEQUENCE,
                        &REFERENCED_SOP_INSTANCE_UID,
                    )
                );
                assert_eq!(result[2].value, "RefSopInstance3");
            }
            Err(e) => {
                panic!("Unable to find all ReferencedSopInstanceUIDs: {e:#?}");
            }
        }
    }

    #[test]
    fn find_nested_elements_level2_by_index() {
        let obj = create_dicom_model();
        match grep(
            &obj,
            &format!(
                "{}[0]/{}[0]/{}",
                &REQUEST_ATTRIBUTES_SEQUENCE, &REFERENCED_STUDY_SEQUENCE, &REFERENCED_SOP_CLASS_UID
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[0]/{}[0]/{}",
                        &REQUEST_ATTRIBUTES_SEQUENCE,
                        &REFERENCED_STUDY_SEQUENCE,
                        &REFERENCED_SOP_CLASS_UID,
                    )
                );
                assert_eq!(result[0].value, "RefSopClass1");
            }
            Err(e) => {
                panic!("Unable to find the first ReferencedSopClassUIDs: {e:#?}");
            }
        }

        match grep(
            &obj,
            &format!(
                "{}[1]/{}[0]/{}",
                &REQUEST_ATTRIBUTES_SEQUENCE, &REFERENCED_STUDY_SEQUENCE, &REFERENCED_SOP_CLASS_UID
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[1]/{}[0]/{}",
                        &REQUEST_ATTRIBUTES_SEQUENCE,
                        &REFERENCED_STUDY_SEQUENCE,
                        &REFERENCED_SOP_CLASS_UID,
                    )
                );
                assert_eq!(result[0].value, "RefSopClass2");
            }
            Err(e) => {
                panic!("Unable to find the second ReferencedSopClassUIDs: {e:#?}");
            }
        }

        match grep(
            &obj,
            &format!(
                "{}[1]/{}[1]/{}",
                &REQUEST_ATTRIBUTES_SEQUENCE, &REFERENCED_STUDY_SEQUENCE, &REFERENCED_SOP_CLASS_UID
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[1]/{}[1]/{}",
                        &REQUEST_ATTRIBUTES_SEQUENCE,
                        &REFERENCED_STUDY_SEQUENCE,
                        &REFERENCED_SOP_CLASS_UID,
                    )
                );
                assert_eq!(result[0].value, "RefSopClass3");
            }
            Err(e) => {
                panic!("Unable to find the third ReferencedSopClassUIDs: {e:#?}");
            }
        }

        //////////////////////////////
        match grep(
            &obj,
            &format!(
                "{}[0]/{}[0]/{}",
                &REQUEST_ATTRIBUTES_SEQUENCE,
                &REFERENCED_STUDY_SEQUENCE,
                &REFERENCED_SOP_INSTANCE_UID
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[0]/{}[0]/{}",
                        &REQUEST_ATTRIBUTES_SEQUENCE,
                        &REFERENCED_STUDY_SEQUENCE,
                        &REFERENCED_SOP_INSTANCE_UID,
                    )
                );
                assert_eq!(result[0].value, "RefSopInstance1");
            }
            Err(e) => {
                panic!("Unable to find the first ReferencedSopInstanceUID: {e:#?}");
            }
        }
        match grep(
            &obj,
            &format!(
                "{}[1]/{}[0]/{}",
                &REQUEST_ATTRIBUTES_SEQUENCE,
                &REFERENCED_STUDY_SEQUENCE,
                &REFERENCED_SOP_INSTANCE_UID
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[1]/{}[0]/{}",
                        &REQUEST_ATTRIBUTES_SEQUENCE,
                        &REFERENCED_STUDY_SEQUENCE,
                        &REFERENCED_SOP_INSTANCE_UID,
                    )
                );
                assert_eq!(result[0].value, "RefSopInstance2");
            }
            Err(e) => {
                panic!("Unable to find the second ReferencedSopInstanceUID: {e:#?}");
            }
        }
        match grep(
            &obj,
            &format!(
                "{}[1]/{}[1]/{}",
                &REQUEST_ATTRIBUTES_SEQUENCE,
                &REFERENCED_STUDY_SEQUENCE,
                &REFERENCED_SOP_INSTANCE_UID
            ),
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].path,
                    format!(
                        "{}[1]/{}[1]/{}",
                        &REQUEST_ATTRIBUTES_SEQUENCE,
                        &REFERENCED_STUDY_SEQUENCE,
                        &REFERENCED_SOP_INSTANCE_UID,
                    )
                );
                assert_eq!(result[0].value, "RefSopInstance3");
            }
            Err(e) => {
                panic!("Unable to find the third ReferencedSopInstanceUID: {e:#?}");
            }
        }
    }
}
