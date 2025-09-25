use crate::Error;
use dicom_core::dictionary::TagRange;
use dicom_core::{DataDictionary, Tag};
use dicom_dictionary_std::StandardDataDictionary;
use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;
use tracing::error;

#[derive(Copy, Clone, Debug, Default)]
pub(crate) enum Selector {
    #[default]
    All,
    Index(usize),
    Range(usize, usize),
}

static RE_SELECTOR_RANGE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+)-(\d+)").unwrap());

impl FromStr for Selector {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "*" {
            Ok(Selector::All)
        } else if s.contains('-') {
            if let Some(caps) = RE_SELECTOR_RANGE.captures(s) {
                let start = caps[1].parse::<usize>().map_err(|e| {
                    error!("Unable to convert value to an usize: {e:#?}");
                    Error::InvalidSelectorRangeFormat
                })?;
                let end = caps[2].parse::<usize>().map_err(|e| {
                    error!("Unable to convert value to an usize: {e:#?}");
                    Error::InvalidSelectorRangeFormat
                })?;
                Ok(Selector::Range(start, end))
            } else {
                Err(Error::InvalidSelectorRangeFormat)
            }
        } else if let Ok(index) = s.parse::<usize>() {
            Ok(Selector::Index(index))
        } else {
            Err(Error::InvalidSelectorFormat)
        }
    }
}

#[derive(Debug)]
pub(crate) struct SearchPattern {
    pub tag: Tag,
    pub selectors: Vec<Selector>,
}

static PATTERN_DICOM_TAG: &str = r"\(([0-9a-fA-F]{4}),([0-9a-fA-F]{4})\).*";
static RE_DICOM_TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(PATTERN_DICOM_TAG).unwrap());

impl FromStr for SearchPattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dict = StandardDataDictionary;

        let tag = if let Some(caps) = RE_DICOM_TAG.captures(s) {
            let group = u16::from_str_radix(&caps[1], 16).map_err(|e| {
                error!("Unable to convert hex value to u16: {e:#?}");
                Error::InvalidGroupFormat
            })?;
            let element = u16::from_str_radix(&caps[2], 16).map_err(|e| {
                error!("Unable to convert hex value to u16: {e:#?}");
                Error::InvalidElementFormat
            })?;
            Ok(Tag(group, element))
        } else {
            // Find a tag by name
            let entry = match s.find('[') {
                None => dict.by_name(s).ok_or(Error::InvalidTagFormat),
                Some(i) => {
                    let name = &s[0..i];
                    dict.by_name(name).ok_or(Error::InvalidTagFormat)
                }
            };
            let is_err = entry.is_err();
            if is_err {
                return Err(entry.unwrap_err());
            }
            let entry = entry.unwrap();
            match entry.tag {
                TagRange::Single(tag) => Ok(tag),
                TagRange::Group100(_) => Err(Error::InvalidTagFormat),
                TagRange::Element100(_) => Err(Error::InvalidTagFormat),
                TagRange::GroupLength => Err(Error::InvalidTagFormat),
                TagRange::PrivateCreator => Err(Error::InvalidTagFormat),
            }
        }?;

        let start = s.find('[');
        let end = s.rfind(']');
        let mut selectors = Vec::new();

        if start.is_some() && end.is_none() {
            error!("Found '[' in the pearch PATTERN but not ']'");
            return Err(Error::InvalidSearchPatternFormat);
        }
        if start.is_none() && end.is_some() {
            error!("Found ']' in the pearch PATTERN but not '['");
            return Err(Error::InvalidSearchPatternFormat);
        }

        if let (Some(start), Some(end)) = (start, end) {
            let sub = &s[start + 1..end];
            for t in sub.split(',') {
                let selector = Selector::from_str(t).map_err(|e| {
                    error!("Invalid selector: {e:#?}");
                    Error::InvalidSearchPatternFormat
                })?;
                selectors.push(selector);
            }
        }

        Ok(SearchPattern { tag, selectors })
    }
}

pub(crate) struct SearchPatterns {
    pub patterns: Vec<SearchPattern>,
}

impl FromStr for SearchPatterns {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tv: Vec<&str> = s.split('/').collect();
        let mut patterns = Vec::with_capacity(tv.len());
        for t in tv {
            match SearchPattern::from_str(t) {
                Ok(value) => {
                    patterns.push(value);
                }
                Err(e) => {
                    error!("Unable to parse search PATTERN: {e:#?}");
                    return Err(Error::InvalidSearchPattern);
                }
            }
        }

        Ok(SearchPatterns { patterns })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector_all() {
        let selector = Selector::from_str("*").unwrap();
        assert!(matches!(selector, Selector::All));
    }

    #[test]
    fn test_selector_index() {
        let selector = Selector::from_str("42").unwrap();
        assert!(matches!(selector, Selector::Index(42)));
    }

    #[test]
    fn test_selector_range() {
        let selector = Selector::from_str("1-5").unwrap();
        assert!(matches!(selector, Selector::Range(1, 5)));
    }

    #[test]
    fn test_invalid_selector() {
        assert!(matches!(
            Selector::from_str("invalid"),
            Err(Error::InvalidSelectorFormat)
        ));
        assert!(matches!(
            Selector::from_str("1-"),
            Err(Error::InvalidSelectorRangeFormat)
        ));
        assert!(matches!(
            Selector::from_str("-5"),
            Err(Error::InvalidSelectorRangeFormat)
        ));
    }

    #[test]
    fn test_search_pattern_valid_tag() {
        let pattern = SearchPattern::from_str("(0008,0060)").unwrap();
        assert_eq!(pattern.tag, Tag(0x0008, 0x0060));
        assert!(pattern.selectors.is_empty());
    }

    #[test]
    fn test_search_pattern_invalid_tag() {
        assert!(matches!(
            SearchPattern::from_str("invalid"),
            Err(Error::InvalidTagFormat)
        ));
        assert!(matches!(
            SearchPattern::from_str("(008,0060)"),
            Err(Error::InvalidTagFormat)
        ));
    }

    #[test]
    fn test_search_pattern_with_selector() {
        let pattern = SearchPattern::from_str("(0008,0060)[1]").unwrap();
        assert_eq!(pattern.tag, Tag(0x0008, 0x0060));
        assert_eq!(pattern.selectors.len(), 1);
        assert!(matches!(pattern.selectors[0], Selector::Index(1)));
    }

    #[test]
    fn test_search_pattern_with_multiple_selectors() {
        let pattern = SearchPattern::from_str("(0008,0060)[1,2-4,*]").unwrap();
        assert_eq!(pattern.tag, Tag(0x0008, 0x0060));
        assert_eq!(pattern.selectors.len(), 3);
        assert!(matches!(pattern.selectors[0], Selector::Index(1)));
        assert!(matches!(pattern.selectors[1], Selector::Range(2, 4)));
        assert!(matches!(pattern.selectors[2], Selector::All));
    }

    #[test]
    fn test_search_pattern_invalid_selector() {
        assert!(matches!(
            SearchPattern::from_str("(0008,0060)[invalid]"),
            Err(Error::InvalidSearchPatternFormat)
        ));
    }

    #[test]
    fn test_search_pattern_mismatched_brackets() {
        assert!(matches!(
            SearchPattern::from_str("(0008,0060)[1"),
            Err(Error::InvalidSearchPatternFormat)
        ));
        assert!(matches!(
            SearchPattern::from_str("(0008,0060)1]"),
            Err(Error::InvalidSearchPatternFormat)
        ));
    }

    #[test]
    fn test_search_pattern_valid_tag_name() {
        let pattern = SearchPattern::from_str("Modality").unwrap();
        assert_eq!(pattern.tag, Tag(0x0008, 0x0060));
        assert!(pattern.selectors.is_empty());
    }

    #[test]
    fn test_search_pattern_tag_name_with_selector() {
        let pattern = SearchPattern::from_str("Modality[1,2-3]").unwrap();
        assert_eq!(pattern.tag, Tag(0x0008, 0x0060));
        assert_eq!(pattern.selectors.len(), 2);
        assert!(matches!(pattern.selectors[0], Selector::Index(1)));
        assert!(matches!(pattern.selectors[1], Selector::Range(2, 3)));
    }

    #[test]
    fn test_search_pattern_invalid_tag_name() {
        assert!(matches!(
            SearchPattern::from_str("NonExistentTag"),
            Err(Error::InvalidTagFormat)
        ));
    }
}
