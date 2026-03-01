use dicom_object::InMemDicomObject;
use crate::{Value, DicomValue};

crate::dicom_value_type!(DA, DA, String);
crate::dicom_value_type!(DAs, DA, Vec<String>);
crate::one_to_many_dicom_value_by_delim!(DA, DAs, '\\');
crate::from_dicom_object_for_string!(DA, DA);
crate::from_dicom_object_for_strings!(DAs, DA, '\\');
crate::dicom_value_from_str!(DA);
crate::dicom_value_from_same_type!(DA, String);
crate::dicom_value_from_same_type!(DAs, Vec<String>);
crate::to_dicom_object_for_string!(DA, DA);
crate::to_dicom_object_for_strings!(DAs, DA);

impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for DA<G, E> {}
impl<const G: u16, const E: u16> DicomValue<InMemDicomObject> for DAs<G, E> {}

const FORMAT: &'static str = "%Y%m%d";
impl<const G: u16, const E: u16> DA<G, E> {
    /// Converts the DICOM date string to a `chrono::NaiveDate`.
    ///
    /// # Returns
    ///
    /// - `Ok(None)` if the value is empty
    /// - `Ok(Some(date))` if the date was successfully parsed
    /// - `Err(ParseError)` if the date string is invalid or does not match the YYYYMMDD format
    ///
    /// # Example
    ///
    /// ```ignore
    /// let da = DA { value: "20260123".to_string()};
    /// let date = da.to_date()?;
    /// assert_eq!(date, Some(NaiveDate::from_ymd_opt(2026, 1, 23).unwrap()));
    /// ```
    pub fn to_date(&self) -> Result<Option<chrono::NaiveDate>, chrono::ParseError> {
        if self.value.is_empty() {
            Ok(None)
        } else {
            Ok(Some(chrono::NaiveDate::parse_from_str(
                &self.value,
                FORMAT,
            )?))
        }
    }
}

impl<const G: u16, const E: u16> DAs<G, E> {
    /// Converts multiple DICOM date strings to a vector of `chrono::NaiveDate`.
    ///
    /// # Returns
    ///
    /// - `Ok(None)` if the value collection is empty
    /// - `Ok(Some(dates))` if all dates were successfully parsed
    /// - `Err(ParseError)` if any date string is invalid or does not match the YYYYMMDD format
    ///
    /// # Example
    ///
    /// ```ignore
    /// let das = DAs::new(vec!["20260123".to_string(), "20260124".to_string()]);
    /// let dates = das.to_dates()?;
    /// assert_eq!(dates.unwrap().len(), 2);
    /// ```
    pub fn to_dates(&self) -> Result<Option<Vec<chrono::NaiveDate>>, chrono::ParseError> {
        if self.value.is_empty() {
            return Ok(None);
        }
        let mut v = vec![];
        for s in &self.value {
            match chrono::NaiveDate::parse_from_str(s, FORMAT) {
                Ok(value) => {
                    v.push(value);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(Some(v))
    }
}
