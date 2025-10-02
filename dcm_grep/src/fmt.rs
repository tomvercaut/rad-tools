use dicom_core::dictionary::DataDictionaryEntry;
use dicom_core::{DataDictionary, Tag};

/// An enumeration representing the formatting type for DICOM tags.
///
/// This enum allows specifying how DICOM tags should be formatted,
/// either by their descriptive `Name` or by their numerical `Tag` value (group and element).
///
/// # Variants
///
/// - `Name`: Formats DICOM tags using their human-readable, commonly used names.
///   For example, "PatientName" or "StudyDate".
///
/// - `Tag`: Formats DICOM tags using their numerical group and element value,
///   typically represented in hexadecimal. For example, "(0010,0010)" for the "Patient Name" tag.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FmtType {
    /// Formats DICOM tags using their human-readable, commonly used names.
    Name,
    /// Formats DICOM tags using their numerical group and element value,
    Tag,
}

/// A trait for formatting instances contains DICOM tag information using a data dictionary.
///
/// This trait provides functionality to format DICOM tags either by their
/// human-readable names or by their numerical tag values, using a data dictionary
/// as a reference.
pub trait ToDictFmtStr {
    /// Formats an instance containing a DICOM tag according to the specified format type.
    ///
    /// # Parameters
    ///
    /// * `dict` - A reference to a DICOM data dictionary implementing the `DataDictionary` trait
    /// * `fmt_type` - The format type (`FmtType::Name` or `FmtType::Tag`) to use for formatting
    ///
    /// # Returns
    ///
    /// Returns a `String` containing either:
    /// - The human-readable name of the tag (if `fmt_type` is `Name` and the tag exists in the dictionary)
    /// - The numerical representation of the tag (in all other cases)
    fn to_dict_fmt_str<D>(&self, dict: &D, fmt_type: FmtType) -> String
    where
        D: DataDictionary;
}

impl ToDictFmtStr for Tag {
    fn to_dict_fmt_str<D>(&self, dict: &D, fmt_type: FmtType) -> String
    where
        D: DataDictionary,
    {
        match fmt_type {
            FmtType::Name => match dict.by_tag(*self) {
                None => self.to_string(),
                Some(entry) => entry.alias().to_string(),
            },
            FmtType::Tag => self.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dicom_dictionary_std::StandardDataDictionary;

    #[test]
    fn fmt_by_name() {
        let dict = StandardDataDictionary;
        let tag = Tag(0x0010, 0x0010); // Patient Name tag
        assert_eq!(tag.to_dict_fmt_str(&dict, FmtType::Name), "PatientName");
    }

    #[test]
    fn fmt_by_tag() {
        let dict = StandardDataDictionary;
        let tag = Tag(0x0010, 0x0010); // Patient Name tag
        assert_eq!(tag.to_dict_fmt_str(&dict, FmtType::Tag), "(0010,0010)");
    }
    #[test]
    fn meta_fmt_by_name() {
        let dict = StandardDataDictionary;
        let tag = Tag(0x0002, 0x0010); // Patient Name tag
        assert_eq!(
            tag.to_dict_fmt_str(&dict, FmtType::Name),
            "TransferSyntaxUID"
        );
    }
}
