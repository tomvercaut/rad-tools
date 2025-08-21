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
                for selector in &pattern.selectors {
                    match selector {
                        Selector::All => {
                            let nested_results = items
                                .iter()
                                .map(|item| grep_matching_elements(item, patterns, ipattern + 1))
                                .collect::<Vec<_>>();
                            for (index, nested_result) in nested_results.into_iter().enumerate() {
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
                                    let nested_result =
                                        grep_matching_elements(sub_item, patterns, ipattern + 1);
                                    append_result(&mut vec, path_prefix, nested_result);
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
