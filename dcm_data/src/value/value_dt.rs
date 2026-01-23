crate::dicom_value_type!(DT, DT, String);
crate::dicom_value_type!(DTs, DT, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(DT, DTs, '\\');
crate::from_dicom_object_for_string!(DT, DT);
crate::from_dicom_object_for_strings!(DTs, DT, '\\');
crate::dicom_value_from_str!(DT);
crate::dicom_value_from_same_type!(DT, String);
crate::dicom_value_from_same_type!(DTs, Vec<String>);

/// Supported DICOM DT (DateTime) format strings for parsing.
/// Formats include:
/// - `%Y%m%d%H%M%S`: Basic datetime format (e.g., "20260123143000" for 2026-01-23 14:30:00)
/// - `%Y%m%d%H%M%S.%f`: Datetime format with fractional seconds (e.g., "20260123143000.123456")
const DATETIME_FORMATS: [&'static str; 2] = ["%Y%m%d%H%M%S", "%Y%m%d%H%M%S.%f"];

/// Attempts to parse a string into a `NaiveDateTime` using supported DICOM DT formats.
///
/// Tries each format in `DATETIME_FORMATS` until one succeeds or all fail.
///
/// # Arguments
/// * `s` - The datetime string to parse in DICOM DT format
///
/// # Returns
/// * `Ok(Some(NaiveDateTime))` if parsing succeeds with any supported format
/// * `Err(ParseError)` if all format attempts fail
fn to_datetime(s: &str) -> Result<Option<chrono::NaiveDateTime>, chrono::ParseError> {
    let mut last_error = None;
    for format in DATETIME_FORMATS {
        match chrono::NaiveDateTime::parse_from_str(s, format) {
            Ok(value) => {
                return Ok(Some(value));
            }
            Err(e) => {
                last_error = Some(e);
            }
        }
    }
    if let Some(e) = last_error {
        Err(e)
    } else {
        unreachable!();
    }
}

impl<const G: u16, const E: u16> DT<G, E> {
    /// Converts the DICOM DT (DateTime) value to a `chrono::NaiveDateTime`.
    ///
    /// # Returns
    /// * `Ok(None)` if the value is an empty string
    /// * `Ok(Some(NaiveDateTime))` if parsing succeeds
    /// * `Err(ParseError)` if the value cannot be parsed with any supported format
    pub fn to_datetime(&self) -> Result<Option<chrono::NaiveDateTime>, chrono::ParseError> {
        if self.value.is_empty() {
            return Ok(None);
        }
        to_datetime(&self.value)
    }
}

impl<const G: u16, const E: u16> DTs<G, E> {
    /// Converts multiple DICOM DT (DateTime) values to a vector of `chrono::NaiveDateTime`.
    ///
    /// Empty string values are skipped and not included in the result vector.
    ///
    /// # Returns
    /// * `Ok(None)` if the value collection is empty
    /// * `Ok(Some(Vec<NaiveDateTime>))` if all values parse successfully
    /// * `Err(ParseError)` if any value cannot be parsed with any supported format
    pub fn to_datetimes(&self) -> Result<Option<Vec<chrono::NaiveDateTime>>, chrono::ParseError> {
        if self.value.is_empty() {
            return Ok(None);
        }
        let mut v = vec![];
        for s in &self.value {
            match to_datetime(s) {
                Ok(value) => {
                    if let Some(value) = value {
                        v.push(value);
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(Some(v))
    }
}
