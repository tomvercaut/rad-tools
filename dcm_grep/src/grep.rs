use crate::Error;
use crate::pattern::{SearchPattern, SearchPatterns, Selector};
use dicom_core::header::Header;
use dicom_core::value::Value;
use dicom_core::{PrimitiveValue, Tag};
use dicom_object::mem::InMemElement;
use dicom_object::{FileMetaTable, InMemDicomObject};
use std::str::FromStr;
use tracing::error;

/// Searches for DICOM elements in the provided DICOM object that match the given search pattern.
///
/// # Arguments
/// * `obj` - The DICOM object to search through
/// * `pattern` - A string representing the search pattern in the format "tag[selector]/tag[selector]/..."
/// * `recursive` - Whether to search recursively through nested sequences
///
/// # Returns
/// * `Ok(Vec<GrepResult>)` - A vector of matching elements with their paths and values
/// * `Err(Error)` - If the pattern is invalid
pub fn grep<'a>(
    obj: &'a InMemDicomObject,
    pattern: &str,
    recursive: bool,
) -> Result<Vec<GrepResult<'a>>, Error> {
    let patterns = SearchPatterns::from_str(pattern).map_err(|e| {
        error!("Unable to create search patterns: {e:#?}");
        Error::InvalidState
    })?;

    let v = grep_matching_elements(obj, &patterns, 0, recursive);
    Ok(v)
}

/// Searches for DICOM elements in the provided file meta table that match the given search pattern.
///
/// # Arguments
/// * `obj` - The DICOM file meta table to search through
/// * `pattern` - A string representing the search pattern in the format
///   "tag[selector]/tag[selector]/...".
///   It's important to note using selectors to query meta elements (with a group tag == 0x0002 and an element tag <= 0x0102)
///   is considered an invalid pattern. Based on the registry of DICOM File Meta Elements on:
///   https://dicom.nema.org/medical/dicom/current/output/chtml/part06/chapter_7.html
///
/// # Returns
/// * `Ok(Vec<GrepMetaResult>)` - A vector of matching meta elements with their paths and primitive values
/// * `Err(Error)` - If the pattern is invalid or contains unsupported meta values
pub fn grep_meta(obj: &FileMetaTable, pattern: &str) -> Result<Vec<GrepMetaResult>, Error> {
    let patterns = SearchPatterns::from_str(pattern).map_err(|e| {
        error!("Unable to create search patterns: {e:#?}");
        Error::InvalidState
    })?;

    let mut v = vec![];
    let npattern = patterns.patterns.len();
    for elem in obj.to_element_iter() {
        let pattern = patterns.patterns.first();
        if pattern.is_none() {
            // No more patterns to match.
            break;
        }
        let element_tag = elem.tag();
        let p = pattern.unwrap();
        if p.tag != element_tag {
            continue;
        }
        // A matching tag has been found in the meta element table.
        // Check if the pattern(s) and selector are valid for a meta element.
        // There should be no other patterns or selectors.
        let mut has_error = false;
        if !p.selectors.is_empty() {
            error!(
                "{:#?} has selectors in the meta element pattern: {:#?}",
                &element_tag, p
            );
            has_error = true;
        }
        if npattern > 1 {
            error!(
                "{:#?} has multiple search patterns in the meta element pattern: {:#?}",
                &element_tag, &patterns.patterns
            );
            has_error = true;
        }
        if has_error {
            return Err(Error::InvalidState);
        }

        // The pattern is valid for a meta element.
        // Check if the value is supported.
        // If it is, add the value to the results.
        let meta_value = match elem.value() {
            Value::Primitive(primitive) => Ok(primitive.clone()),
            _ => {
                error!(
                    "{:#?} has an unsupported value representation meta element: {:#?}",
                    &element_tag,
                    elem.vr()
                );
                Err(Error::UnsupportedMetaValue)
            }
        }?;
        v.push(GrepMetaResult {
            tag: element_tag,
            value: meta_value,
        });
    }
    Ok(v)
}

/// Represents a matched DICOM element from a grep search operation
#[derive(Debug)]
pub struct GrepResult<'a> {
    /// Path of the element value in the DICOM object.
    /// Example: (1234,5678)[2]/(9876,5432)
    pub search_pattern: Vec<SearchPattern>,
    /// DICOM element
    pub element: &'a InMemElement,
}

/// Represents a matched DICOM meta element from a grep search operation
#[derive(Debug)]
pub struct GrepMetaResult {
    /// DICOM tag.
    /// Example: (1234,5678)
    pub tag: Tag,
    /// DICOM element
    pub value: PrimitiveValue,
}

/// Converts a DICOM element's value to a string representation
///
/// # Arguments
/// * `element` - The DICOM element to convert
///
/// # Returns
/// A string representation of the element's value:
/// * For primitive values - their string representation
/// * For sequences - the text "Sequence"
/// * For pixel sequences - the text "PixelSequence"
pub fn element_value_to_string(element: &InMemElement) -> String {
    match element.value() {
        Value::Primitive(value) => value.to_string(),
        Value::Sequence(_) => "Sequence".to_string(),
        Value::PixelSequence(_) => "PixelSequence".to_string(),
    }
}

/// Recursively searches for DICOM elements that match the given patterns
///
/// # Arguments
/// * `obj` - The DICOM object to search through
/// * `patterns` - The parsed search patterns to match against
/// * `ipattern` - The current pattern index being processed
/// * `recursive` - Whether to search recursively through nested sequences
///
/// # Returns
/// A vector of GrepResult containing matching elements and their paths
fn grep_matching_elements<'a>(
    obj: &'a InMemDicomObject,
    patterns: &SearchPatterns,
    ipattern: usize,
    recursive: bool,
) -> Vec<GrepResult<'a>> {
    let pattern = &patterns.patterns[ipattern];
    let mut vec = vec![];
    if let Ok(element) = obj.element(pattern.tag) {
        match element.value() {
            Value::Primitive(_primitive) => {
                vec.push(GrepResult {
                    search_pattern: vec![pattern.clone()],
                    element,
                });
            }
            Value::Sequence(seq) => {
                let items = seq.items();

                if pattern.selectors.is_empty() {
                    vec.push(GrepResult {
                        search_pattern: vec![pattern.clone()],
                        element,
                    });
                } else {
                    for selector in &pattern.selectors {
                        match selector {
                            Selector::All => {
                                let nested_results = items
                                    .iter()
                                    .map(|item| {
                                        grep_matching_elements(
                                            item,
                                            patterns,
                                            ipattern + 1,
                                            recursive,
                                        )
                                    })
                                    .collect::<Vec<_>>();
                                for (index, nested_result) in nested_results.into_iter().enumerate()
                                {
                                    append_result2(
                                        &mut vec,
                                        &SearchPattern {
                                            tag: pattern.tag,
                                            selectors: vec![Selector::Index(index)],
                                        },
                                        nested_result,
                                    );
                                }
                            }
                            Selector::Index(index) => {
                                if let Some(sub_item) = items.get(*index) {
                                    let nested_result = grep_matching_elements(
                                        sub_item,
                                        patterns,
                                        ipattern + 1,
                                        recursive,
                                    );
                                    append_result2(
                                        &mut vec,
                                        &SearchPattern {
                                            tag: pattern.tag,
                                            selectors: vec![Selector::Index(*index)],
                                        },
                                        nested_result,
                                    );
                                }
                            }
                            Selector::Range(start, end) => {
                                for index in *start..*end {
                                    if let Some(sub_item) = items.get(index) {
                                        let nested_result = grep_matching_elements(
                                            sub_item,
                                            patterns,
                                            ipattern + 1,
                                            recursive,
                                        );
                                        append_result2(
                                            &mut vec,
                                            &SearchPattern {
                                                tag: pattern.tag,
                                                selectors: vec![Selector::Range(*start, *end)],
                                            },
                                            nested_result,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }

                if recursive {
                    for (index, item) in items.iter().enumerate() {
                        // When recursive, the pattern index is reset to 0, so the pattern can be discovered deeper in the tree.
                        let nested_results = grep_matching_elements(item, patterns, 0, recursive);
                        append_result2(
                            &mut vec,
                            &SearchPattern {
                                tag: pattern.tag,
                                selectors: vec![Selector::Index(index)],
                            },
                            nested_results,
                        );
                    }
                }
            }
            Value::PixelSequence(_) => vec.push(GrepResult {
                search_pattern: vec![SearchPattern {
                    tag: pattern.tag,
                    selectors: vec![],
                }],
                element,
            }),
        }
    }

    if recursive {
        for element in obj {
            match element.value() {
                Value::Sequence(seq) => {
                    let items = seq.items();
                    for (index, item) in items.iter().enumerate() {
                        // When recursive, the pattern index is reset to 0, so the pattern can be discovered deeper in the tree.
                        let nested_results = grep_matching_elements(item, patterns, 0, recursive);
                        append_result2(
                            &mut vec,
                            &SearchPattern {
                                tag: element.tag(),
                                selectors: vec![Selector::Index(index)],
                            },
                            nested_results,
                        );
                    }
                }
                _ => {
                    // Not needed otherwise it would have been found in the previous if block.
                    // We only want to find elements that are in a sequence.
                }
            }
        }
    }
    vec
}

/// Appends nested search results to the main results vector with updated search patterns.
///
/// # Arguments
/// * `vec` - The vector to append the results to
/// * `search_pattern` - The search pattern that will be prepend to the search pattern of each result
/// * `results` - The nested results to append
fn append_result2<'a>(
    vec: &mut Vec<GrepResult<'a>>,
    search_pattern: &SearchPattern,
    results: Vec<GrepResult<'a>>,
) {
    for fr in results {
        let mut sp = Vec::with_capacity(fr.search_pattern.len() + 1);
        sp.push(search_pattern.clone());
        for p in &fr.search_pattern {
            sp.push(p.clone());
        }
        let nfr = GrepResult {
            search_pattern: sp,
            element: fr.element,
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
        IMPLEMENTATION_CLASS_UID, IMPLEMENTATION_VERSION_NAME, MEDIA_STORAGE_SOP_CLASS_UID,
        MEDIA_STORAGE_SOP_INSTANCE_UID, PATIENT_ID, PATIENT_NAME, REFERENCED_SOP_CLASS_UID,
        REFERENCED_SOP_INSTANCE_UID, REFERENCED_STUDY_SEQUENCE, REQUEST_ATTRIBUTES_SEQUENCE,
        SERIES_DATE, SERIES_TIME, SOP_CLASS_UID, STUDY_DATE, STUDY_TIME, TRANSFER_SYNTAX_UID,
    };
    use dicom_dictionary_std::uids::{CT_IMAGE_STORAGE, IMPLICIT_VR_LITTLE_ENDIAN};
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
        match grep(&obj, &SOP_CLASS_UID.to_string(), false) {
            Ok(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].search_pattern[0].tag, SOP_CLASS_UID);
                assert_eq!(
                    element_value_to_string(results[0].element),
                    CT_IMAGE_STORAGE
                );
            }
            Err(e) => panic!("Unable to find SOP_CLASS_UID: {e:#?}"),
        }
        match grep(&obj, &PATIENT_ID.to_string(), false) {
            Ok(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].search_pattern[0].tag, PATIENT_ID);
                assert_eq!(element_value_to_string(results[0].element), "123456");
            }
            Err(e) => panic!("Unable to find PATIENT_ID: {e:#?}"),
        }
        match grep(&obj, &PATIENT_NAME.to_string(), false) {
            Ok(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].search_pattern[0].tag, PATIENT_NAME);
                assert_eq!(element_value_to_string(results[0].element), "Last^First");
            }
            Err(e) => panic!("Unable to find PATIENT_NAME: {e:#?}"),
        }
        match grep(&obj, &STUDY_DATE.to_string(), false) {
            Ok(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].search_pattern[0].tag, STUDY_DATE);
                assert_eq!(element_value_to_string(results[0].element), "20230102");
            }
            Err(e) => panic!("Unable to find STUDY_DATE: {e:#?}"),
        }
        match grep(&obj, &STUDY_TIME.to_string(), false) {
            Ok(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].search_pattern[0].tag, STUDY_TIME);
                assert_eq!(element_value_to_string(results[0].element), "171657.858653");
            }
            Err(e) => panic!("Unable to find STUDY_TIME: {e:#?}"),
        }
        match grep(&obj, &SERIES_DATE.to_string(), false) {
            Ok(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].search_pattern[0].tag, SERIES_DATE);
                assert_eq!(element_value_to_string(results[0].element), "20230102");
            }
            Err(e) => panic!("Unable to find SERIES_DATE: {e:#?}"),
        }
        match grep(&obj, &SERIES_TIME.to_string(), false) {
            Ok(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].search_pattern[0].tag, SERIES_TIME);
                assert_eq!(element_value_to_string(results[0].element), "173657.158653");
            }
            Err(e) => panic!("Unable to find SERIES_TIME: {e:#?}"),
        }
    }

    #[test]
    fn find_sequence_level1() {
        let obj = create_dicom_model();
        match grep(&obj, &CTDI_PHANTOM_TYPE_CODE_SEQUENCE.to_string(), false) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].search_pattern[0].tag,
                    CTDI_PHANTOM_TYPE_CODE_SEQUENCE
                );
                assert_eq!(element_value_to_string(result[0].element), "Sequence");
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
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 2);
                assert_eq!(result[0].search_pattern[0].tag, REQUEST_ATTRIBUTES_SEQUENCE);
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(0));
                assert_eq!(result[0].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(element_value_to_string(result[0].element), "Sequence");
                assert_eq!(result[1].search_pattern[0].tag, REQUEST_ATTRIBUTES_SEQUENCE);
                assert_eq!(result[1].search_pattern[0].selectors[0], Selector::Index(1));
                assert_eq!(result[1].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(element_value_to_string(result[1].element), "Sequence");
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
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 2);
                assert_eq!(
                    result[0].search_pattern[0].tag,
                    CTDI_PHANTOM_TYPE_CODE_SEQUENCE
                );
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(0));
                assert_eq!(result[0].search_pattern[1].tag, CODE_VALUE);
                assert_eq!(element_value_to_string(result[0].element), "123456");
                assert_eq!(
                    result[1].search_pattern[0].tag,
                    CTDI_PHANTOM_TYPE_CODE_SEQUENCE
                );
                assert_eq!(result[1].search_pattern[0].selectors[0], Selector::Index(1));
                assert_eq!(result[1].search_pattern[1].tag, CODE_VALUE);
                assert_eq!(element_value_to_string(result[1].element), "654321");
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
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 2);
                assert_eq!(
                    result[0].search_pattern[0].tag,
                    CTDI_PHANTOM_TYPE_CODE_SEQUENCE
                );
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(0));
                assert_eq!(result[0].search_pattern[1].tag, CODING_SCHEME_DESIGNATOR);
                assert_eq!(element_value_to_string(result[0].element), "DCM1");
                assert_eq!(
                    result[1].search_pattern[0].tag,
                    CTDI_PHANTOM_TYPE_CODE_SEQUENCE
                );
                assert_eq!(result[1].search_pattern[0].selectors[0], Selector::Index(1));
                assert_eq!(result[1].search_pattern[1].tag, CODING_SCHEME_DESIGNATOR);
                assert_eq!(element_value_to_string(result[1].element), "DCM2");
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
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 2);
                assert_eq!(
                    result[0].search_pattern[0].tag,
                    CTDI_PHANTOM_TYPE_CODE_SEQUENCE
                );
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(0));
                assert_eq!(result[0].search_pattern[1].tag, CODE_MEANING);
                assert_eq!(
                    element_value_to_string(result[0].element),
                    "Coding Meaning1"
                );
                assert_eq!(
                    result[1].search_pattern[0].tag,
                    CTDI_PHANTOM_TYPE_CODE_SEQUENCE
                );
                assert_eq!(result[1].search_pattern[0].selectors[0], Selector::Index(1));
                assert_eq!(result[1].search_pattern[1].tag, CODE_MEANING);

                assert_eq!(
                    element_value_to_string(result[1].element),
                    "Coding Meaning2"
                );
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
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].search_pattern[0].tag,
                    CTDI_PHANTOM_TYPE_CODE_SEQUENCE
                );
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(0));
                assert_eq!(result[0].search_pattern[1].tag, CODE_VALUE);
                assert_eq!(element_value_to_string(result[0].element), "123456");
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
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].search_pattern[0].tag,
                    CTDI_PHANTOM_TYPE_CODE_SEQUENCE
                );
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(1));
                assert_eq!(result[0].search_pattern[1].tag, CODE_VALUE);

                assert_eq!(element_value_to_string(result[0].element), "654321");
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
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].search_pattern[0].tag,
                    CTDI_PHANTOM_TYPE_CODE_SEQUENCE
                );
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(0));
                assert_eq!(result[0].search_pattern[1].tag, CODING_SCHEME_DESIGNATOR);
                assert_eq!(element_value_to_string(result[0].element), "DCM1");
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
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].search_pattern[0].tag,
                    CTDI_PHANTOM_TYPE_CODE_SEQUENCE
                );
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(1));
                assert_eq!(result[0].search_pattern[1].tag, CODING_SCHEME_DESIGNATOR);
                assert_eq!(element_value_to_string(result[0].element), "DCM2");
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
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].search_pattern[0].tag,
                    CTDI_PHANTOM_TYPE_CODE_SEQUENCE
                );
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(0));
                assert_eq!(result[0].search_pattern[1].tag, CODE_MEANING);
                assert_eq!(
                    element_value_to_string(result[0].element),
                    "Coding Meaning1"
                );
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
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(
                    result[0].search_pattern[0].tag,
                    CTDI_PHANTOM_TYPE_CODE_SEQUENCE
                );
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(1));
                assert_eq!(result[0].search_pattern[1].tag, CODE_MEANING);
                assert_eq!(
                    element_value_to_string(result[0].element),
                    "Coding Meaning2"
                );
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
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 3);
                assert_eq!(result[0].search_pattern[0].tag, REQUEST_ATTRIBUTES_SEQUENCE);
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(0));
                assert_eq!(result[0].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(result[0].search_pattern[1].selectors[0], Selector::Index(0));
                assert_eq!(result[0].search_pattern[2].tag, REFERENCED_SOP_CLASS_UID);
                assert_eq!(element_value_to_string(result[0].element), "RefSopClass1");

                assert_eq!(result[1].search_pattern[0].tag, REQUEST_ATTRIBUTES_SEQUENCE);
                assert_eq!(result[1].search_pattern[0].selectors[0], Selector::Index(1));
                assert_eq!(result[1].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(result[1].search_pattern[1].selectors[0], Selector::Index(0));
                assert_eq!(result[1].search_pattern[2].tag, REFERENCED_SOP_CLASS_UID);
                assert_eq!(element_value_to_string(result[1].element), "RefSopClass2");

                assert_eq!(result[2].search_pattern[0].tag, REQUEST_ATTRIBUTES_SEQUENCE);
                assert_eq!(result[2].search_pattern[0].selectors[0], Selector::Index(1));
                assert_eq!(result[2].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(result[2].search_pattern[1].selectors[0], Selector::Index(1));
                assert_eq!(result[2].search_pattern[2].tag, REFERENCED_SOP_CLASS_UID);
                assert_eq!(element_value_to_string(result[2].element), "RefSopClass3");
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
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 3);
                assert_eq!(result[0].search_pattern[0].tag, REQUEST_ATTRIBUTES_SEQUENCE);
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(0));
                assert_eq!(result[0].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(result[0].search_pattern[1].selectors[0], Selector::Index(0));
                assert_eq!(result[0].search_pattern[2].tag, REFERENCED_SOP_INSTANCE_UID);
                assert_eq!(
                    element_value_to_string(result[0].element),
                    "RefSopInstance1"
                );

                assert_eq!(result[1].search_pattern[0].tag, REQUEST_ATTRIBUTES_SEQUENCE);
                assert_eq!(result[1].search_pattern[0].selectors[0], Selector::Index(1));
                assert_eq!(result[1].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(result[1].search_pattern[1].selectors[0], Selector::Index(0));
                assert_eq!(result[1].search_pattern[2].tag, REFERENCED_SOP_INSTANCE_UID);
                assert_eq!(
                    element_value_to_string(result[1].element),
                    "RefSopInstance2"
                );

                assert_eq!(result[2].search_pattern[0].tag, REQUEST_ATTRIBUTES_SEQUENCE);
                assert_eq!(result[2].search_pattern[0].selectors[0], Selector::Index(1));
                assert_eq!(result[2].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(result[2].search_pattern[1].selectors[0], Selector::Index(1));
                assert_eq!(result[2].search_pattern[2].tag, REFERENCED_SOP_INSTANCE_UID);
                assert_eq!(
                    element_value_to_string(result[2].element),
                    "RefSopInstance3"
                );
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
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(result[0].search_pattern[0].tag, REQUEST_ATTRIBUTES_SEQUENCE);
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(0));
                assert_eq!(result[0].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(result[0].search_pattern[1].selectors[0], Selector::Index(0));
                assert_eq!(result[0].search_pattern[2].tag, REFERENCED_SOP_CLASS_UID);
                assert_eq!(element_value_to_string(result[0].element), "RefSopClass1");
            }
            Err(e) => {
                panic!("Unable to find the first ReferencedSopClassUID: {e:#?}");
            }
        }

        match grep(
            &obj,
            &format!(
                "{}[1]/{}[0]/{}",
                &REQUEST_ATTRIBUTES_SEQUENCE, &REFERENCED_STUDY_SEQUENCE, &REFERENCED_SOP_CLASS_UID
            ),
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(result[0].search_pattern[0].tag, REQUEST_ATTRIBUTES_SEQUENCE);
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(1));
                assert_eq!(result[0].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(result[0].search_pattern[1].selectors[0], Selector::Index(0));
                assert_eq!(result[0].search_pattern[2].tag, REFERENCED_SOP_CLASS_UID);
                assert_eq!(element_value_to_string(result[0].element), "RefSopClass2");
            }
            Err(e) => {
                panic!("Unable to find the second ReferencedSopClassUID: {e:#?}");
            }
        }

        match grep(
            &obj,
            &format!(
                "{}[1]/{}[1]/{}",
                &REQUEST_ATTRIBUTES_SEQUENCE, &REFERENCED_STUDY_SEQUENCE, &REFERENCED_SOP_CLASS_UID
            ),
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(result[0].search_pattern[0].tag, REQUEST_ATTRIBUTES_SEQUENCE);
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(1));
                assert_eq!(result[0].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(result[0].search_pattern[1].selectors[0], Selector::Index(1));
                assert_eq!(result[0].search_pattern[2].tag, REFERENCED_SOP_CLASS_UID);
                assert_eq!(element_value_to_string(result[0].element), "RefSopClass3");
            }
            Err(e) => {
                panic!("Unable to find the third ReferencedSopClassUID: {e:#?}");
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
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(result[0].search_pattern[0].tag, REQUEST_ATTRIBUTES_SEQUENCE);
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(0));
                assert_eq!(result[0].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(result[0].search_pattern[1].selectors[0], Selector::Index(0));
                assert_eq!(result[0].search_pattern[2].tag, REFERENCED_SOP_INSTANCE_UID);
                assert_eq!(
                    element_value_to_string(result[0].element),
                    "RefSopInstance1"
                );
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
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(result[0].search_pattern[0].tag, REQUEST_ATTRIBUTES_SEQUENCE);
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(1));
                assert_eq!(result[0].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(result[0].search_pattern[1].selectors[0], Selector::Index(0));
                assert_eq!(result[0].search_pattern[2].tag, REFERENCED_SOP_INSTANCE_UID);
                assert_eq!(
                    element_value_to_string(result[0].element),
                    "RefSopInstance2"
                );
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
            false,
        ) {
            Ok(result) => {
                assert_eq!(result.len(), 1);
                assert_eq!(result[0].search_pattern[0].tag, REQUEST_ATTRIBUTES_SEQUENCE);
                assert_eq!(result[0].search_pattern[0].selectors[0], Selector::Index(1));
                assert_eq!(result[0].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(result[0].search_pattern[1].selectors[0], Selector::Index(1));
                assert_eq!(result[0].search_pattern[2].tag, REFERENCED_SOP_INSTANCE_UID);
                assert_eq!(
                    element_value_to_string(result[0].element),
                    "RefSopInstance3"
                );
            }
            Err(e) => {
                panic!("Unable to find the third ReferencedSopInstanceUID: {e:#?}");
            }
        }
    }

    #[test]
    fn find_nested_elements_recursive() {
        let obj = create_dicom_model();
        match grep(&obj, &REFERENCED_SOP_CLASS_UID.to_string(), true) {
            Ok(results) => {
                assert_eq!(results.len(), 3);
                assert_eq!(
                    results[0].search_pattern[0].tag,
                    REQUEST_ATTRIBUTES_SEQUENCE
                );
                assert_eq!(
                    results[0].search_pattern[0].selectors[0],
                    Selector::Index(0)
                );
                assert_eq!(results[0].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(
                    results[0].search_pattern[1].selectors[0],
                    Selector::Index(0)
                );
                assert_eq!(results[0].search_pattern[2].tag, REFERENCED_SOP_CLASS_UID);
                assert_eq!(element_value_to_string(results[0].element), "RefSopClass1");

                assert_eq!(
                    results[1].search_pattern[0].tag,
                    REQUEST_ATTRIBUTES_SEQUENCE
                );
                assert_eq!(
                    results[1].search_pattern[0].selectors[0],
                    Selector::Index(1)
                );
                assert_eq!(results[1].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(
                    results[1].search_pattern[1].selectors[0],
                    Selector::Index(0)
                );
                assert_eq!(results[1].search_pattern[2].tag, REFERENCED_SOP_CLASS_UID);
                assert_eq!(element_value_to_string(results[1].element), "RefSopClass2");

                assert_eq!(
                    results[2].search_pattern[0].tag,
                    REQUEST_ATTRIBUTES_SEQUENCE
                );
                assert_eq!(
                    results[2].search_pattern[0].selectors[0],
                    Selector::Index(1)
                );
                assert_eq!(results[2].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(
                    results[2].search_pattern[1].selectors[0],
                    Selector::Index(1)
                );
                assert_eq!(results[2].search_pattern[2].tag, REFERENCED_SOP_CLASS_UID);
                assert_eq!(element_value_to_string(results[2].element), "RefSopClass3");
            }
            Err(e) => panic!("Unable to recursively find ReferencedSopClassUIDs: {e:#?}"),
        }

        match grep(&obj, &REFERENCED_SOP_INSTANCE_UID.to_string(), true) {
            Ok(results) => {
                assert_eq!(results.len(), 3);
                assert_eq!(
                    results[0].search_pattern[0].tag,
                    REQUEST_ATTRIBUTES_SEQUENCE
                );
                assert_eq!(
                    results[0].search_pattern[0].selectors[0],
                    Selector::Index(0)
                );
                assert_eq!(results[0].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(
                    results[0].search_pattern[1].selectors[0],
                    Selector::Index(0)
                );
                assert_eq!(
                    results[0].search_pattern[2].tag,
                    REFERENCED_SOP_INSTANCE_UID
                );
                assert_eq!(
                    element_value_to_string(results[0].element),
                    "RefSopInstance1"
                );

                assert_eq!(
                    results[1].search_pattern[0].tag,
                    REQUEST_ATTRIBUTES_SEQUENCE
                );
                assert_eq!(
                    results[1].search_pattern[0].selectors[0],
                    Selector::Index(1)
                );
                assert_eq!(results[1].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(
                    results[1].search_pattern[1].selectors[0],
                    Selector::Index(0)
                );
                assert_eq!(
                    results[1].search_pattern[2].tag,
                    REFERENCED_SOP_INSTANCE_UID
                );
                assert_eq!(
                    element_value_to_string(results[1].element),
                    "RefSopInstance2"
                );

                assert_eq!(
                    results[2].search_pattern[0].tag,
                    REQUEST_ATTRIBUTES_SEQUENCE
                );
                assert_eq!(
                    results[2].search_pattern[0].selectors[0],
                    Selector::Index(1)
                );
                assert_eq!(results[2].search_pattern[1].tag, REFERENCED_STUDY_SEQUENCE);
                assert_eq!(
                    results[2].search_pattern[1].selectors[0],
                    Selector::Index(1)
                );
                assert_eq!(
                    results[2].search_pattern[2].tag,
                    REFERENCED_SOP_INSTANCE_UID
                );
                assert_eq!(
                    element_value_to_string(results[2].element),
                    "RefSopInstance3"
                );
            }
            Err(e) => panic!("Unable to recursively find ReferencedSopInstanceUIDs: {e:#?}"),
        }
    }

    fn create_dicom_meta_table() -> FileMetaTable {
        let meta = FileMetaTableBuilder::new()
            .transfer_syntax(IMPLICIT_VR_LITTLE_ENDIAN)
            .media_storage_sop_class_uid(CT_IMAGE_STORAGE)
            .media_storage_sop_instance_uid("1.2.3.4.5.6")
            .implementation_class_uid("1.2.3.4.5")
            .implementation_version_name("VERSION_01");
        meta.build().unwrap()
    }
    #[test]
    fn test_grep_meta_find_elements() {
        let meta = create_dicom_meta_table();

        fn assert_single_meta(
            meta: &FileMetaTable,
            pattern: &str,
            expected_value: &str,
            label: &str,
        ) {
            match grep_meta(meta, pattern) {
                Ok(matches) => {
                    assert_eq!(matches.len(), 1, "Expected exactly one match for {}", label);
                    let first = &matches[0];
                    assert_eq!(
                        first.tag.to_string(),
                        pattern,
                        "Unexpected path for {}",
                        label
                    );
                    assert_eq!(
                        first.value.to_string(),
                        expected_value,
                        "Unexpected value for {}",
                        label
                    );
                }
                Err(e) => panic!("Unable to find {}: {e:#?}", label),
            }
        }

        assert_single_meta(
            &meta,
            &TRANSFER_SYNTAX_UID.to_string(),
            IMPLICIT_VR_LITTLE_ENDIAN,
            "TransferSyntaxUID",
        );
        assert_single_meta(
            &meta,
            &MEDIA_STORAGE_SOP_CLASS_UID.to_string(),
            CT_IMAGE_STORAGE,
            "MediaStorageSOPClassUID",
        );
        assert_single_meta(
            &meta,
            &MEDIA_STORAGE_SOP_INSTANCE_UID.to_string(),
            "1.2.3.4.5.6",
            "MediaStorageSOPInstanceUID",
        );
        assert_single_meta(
            &meta,
            &IMPLEMENTATION_CLASS_UID.to_string(),
            "1.2.3.4.5",
            "ImplementationClassUID",
        );
        assert_single_meta(
            &meta,
            &IMPLEMENTATION_VERSION_NAME.to_string(),
            "VERSION_01",
            "ImplementationVersionName",
        );
    }

    #[test]
    fn test_grep_meta_invalid_pattern() {
        let meta = create_dicom_meta_table();
        match grep_meta(&meta, "invalid_pattern") {
            Ok(_) => panic!("Expected error for an invalid pattern"),
            Err(Error::InvalidState) => (),
            Err(e) => panic!("Unexpected error: {e:#?}"),
        }

        // Test invalid pattern with selector
        match grep_meta(&meta, &format!("{}[0]", IMPLEMENTATION_CLASS_UID)) {
            Ok(_) => panic!("Expected error for a pattern with selector"),
            Err(Error::InvalidState) => (),
            Err(e) => panic!("Unexpected error: {e:#?}"),
        }

        // Test multiple pattern elements
        match grep_meta(
            &meta,
            &format!(
                "{}/{}",
                IMPLEMENTATION_CLASS_UID, IMPLEMENTATION_VERSION_NAME
            ),
        ) {
            Ok(_) => panic!("Expected error for multiple pattern elements"),
            Err(Error::InvalidState) => (),
            Err(e) => panic!("Unexpected error: {e:#?}"),
        }

        // Test invalid selector range
        match grep_meta(&meta, &format!("{}[0-1]", IMPLEMENTATION_CLASS_UID)) {
            Ok(_) => panic!("Expected error for a pattern with selector range"),
            Err(Error::InvalidState) => (),
            Err(e) => panic!("Unexpected error: {e:#?}"),
        }
    }
}
