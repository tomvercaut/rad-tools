use crate::io::DcmIOError;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use dicom_core::value::Value;
use dicom_core::Tag;
use num_traits::NumCast;

/// Converts a DICOM element to a string representation.
///
/// This function takes a reference to a [`dicom_object::DefaultDicomObject`] and a
/// [`Tag`], and attempts to retrieve the element corresponding to the tag,
/// then converts the element to a string. If successful, the string representation of the
/// element is returned; otherwise, an error of type [`DcmIOError`] is returned.
///
/// # Arguments
///
/// * `obj` - A reference to a DICOM object from which to retrieve the element.
/// * `tag` - The tag of the element to be retrieved.
///
/// # Returns
///
/// * `Ok(String)` - The string representation of the DICOM element.
/// * `Err(DcmIOError)` - An error if the element could not be retrieved or converted to a string.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] if the element retrieval or conversion to a string fails.
/// ```
pub(crate) fn to_string(
    obj: &dicom_object::InMemDicomObject,
    tag: Tag,
) -> Result<String, DcmIOError> {
    Ok(obj.element(tag)?.to_str()?.to_string())
}

/// Converts a DICOM element to an optional string representation.
///
/// This function attempts to retrieve the element from the given DICOM object
/// corresponding to the provided tag and then converts the element to a string.
/// If the element is found and successfully converted, `Some(String)` is returned.
/// If the element is not found, `None` is returned. If an error occurs during
/// retrieval or conversion, an error of type [`DcmIOError`] is returned.
///
/// # Arguments
///
/// * `obj` - A reference to a DICOM object from which to retrieve the element.
/// * `tag` - The tag of the element to be retrieved.
///
/// # Returns
///
/// * `Ok(Some(String))` - The string representation of the DICOM element, if found.
/// * `Ok(None)` - If the element is not found.
/// * `Err(DcmIOError)` - An error if the element could not be retrieved or converted to a string.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] if the element retrieval or conversion to a string fails.
pub(crate) fn to_string_opt(
    obj: &dicom_object::InMemDicomObject,
    tag: Tag,
) -> Result<Option<String>, DcmIOError> {
    match obj.element_opt(tag) {
        Ok(o) => match o {
            None => Ok(None),
            Some(elem) => Ok(Some(elem.to_str()?.to_string())),
        },
        Err(e) => Err(DcmIOError::from(e)),
    }
}

/// Converts a DICOM element to a vector of string representations.
///
/// This function retrieves the element corresponding to the provided tag
/// from the given DICOM object and converts it to a string. The string is
/// then split on backslashes (`\`), with each segment stored in a vector.
///
/// # Arguments
///
/// * `obj` - A reference to a DICOM object from which to retrieve the element.
/// * `tag` - The tag of the element to be retrieved.
///
/// # Returns
///
/// * `Ok(Vec<String>)` - A vector of string representations of the DICOM element.
/// * `Err(DcmIOError)` - An error if the element could not be retrieved or
///   converted to a string.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] if the element retrieval or
/// conversion to a string fails.
pub(crate) fn to_strings(
    obj: &dicom_object::InMemDicomObject,
    tag: Tag,
) -> Result<Vec<String>, DcmIOError> {
    let s = to_string(obj, tag)?;
    Ok(s.split('\\').map(|x| x.to_string()).collect())
}

/// Converts a DICOM element to an optional vector of string representations.
///
/// This function attempts to retrieve the element from the given DICOM object
/// corresponding to the provided tag and then converts the element to a vector
/// of strings. Each string within the vector represents a component of the
/// DICOM element's multivalued string.
///
/// # Arguments
///
/// * `obj` - A reference to a DICOM object from which to retrieve the element.
/// * `tag` - The tag of the element to be retrieved.
///
/// # Returns
///
/// * `Ok(Some(Vec<String>))` - A vector of string representations of the DICOM element, if found.
/// * `Ok(None)` - If the element is not found.
/// * `Err(DcmIOError)` - An error if the element could not be retrieved or converted to a string.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] if the element retrieval or conversion to a string fails.
pub(crate) fn to_strings_opt(
    obj: &dicom_object::InMemDicomObject,
    tag: Tag,
) -> Result<Option<Vec<String>>, DcmIOError> {
    let element = obj.element_opt(tag)?;
    match element {
        None => Ok(None),
        Some(element) => Ok(Some(element.to_multi_str()?.to_vec())),
    }
}

/// Reads and parses a combined date and time from a DICOM object.
///
/// This function retrieves the date and time elements from a DICOM object,
/// combines them, and parses them into a [`NaiveDateTime`].
///
/// # Arguments
///
/// * `obj` - A reference to the DICOM object from which to retrieve the date and time elements.
/// * `tag_date` - The tag of the date element to be retrieved.
/// * `tag_time` - The tag of the time element to be retrieved.
///
/// # Returns
///
/// * `Ok(NaiveDateTime)` - The parsed date and time.
/// * `Err(DcmIOError)` - An error if the date or time elements could not be retrieved or if the parsing fails.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] if the retrieval of either date or time element fails,
/// or if the parsing into `NaiveDateTime` fails.
pub(crate) fn da_tm_to_ndt(
    obj: &dicom_object::InMemDicomObject,
    tag_date: Tag,
    tag_time: Tag,
) -> Result<NaiveDateTime, DcmIOError> {
    let date = to_string(obj, tag_date)?;
    let time = to_string(obj, tag_time)?;
    let dt = NaiveDateTime::parse_from_str(&format!("{}{}", date, time), "%Y%m%d%H%M%S%.f")?;
    Ok(dt)
}

/// Reads and parses optional combined date and time from a DICOM object.
///
/// This function retrieves the date and time elements from a DICOM object by their tags. The elements are optional,
/// meaning they may or may not be present in the DICOM object.
/// * If both elements are found, they are combined and parsed into a [`NaiveDateTime`].
/// * If the date is not empty but the time is, the date is returned with the time set to 00:00:00.
/// * If the date is empty but the time is not, an error `DcmIOError::InvalidDateTimeEmpytDate` is returned.
/// * If either element is missing, `Ok(None)` is returned.
///
/// # Arguments
///
/// * `obj` - A reference to the DICOM object from which to retrieve the date and time elements.
/// * `tag_date` - The tag of the date element to be retrieved.
/// * `tag_time` - The tag of the time element to be retrieved.
///
/// # Returns
///
/// * `Ok(Some(NaiveDateTime))` - The parsed date and time, if both elements are found.
/// * `Ok(None)` - If either the date or time element is missing.
/// * `Err(DcmIOError)` - An error if the date or time elements could not be retrieved or if the parsing fails.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] if the parsing into `NaiveDateTime` fails.
pub(crate) fn da_tm_to_ndt_opt(
    obj: &dicom_object::InMemDicomObject,
    tag_date: Tag,
    tag_time: Tag,
) -> Result<Option<NaiveDateTime>, DcmIOError> {
    let date = to_string_opt(obj, tag_date)?;
    let time = to_string_opt(obj, tag_time)?;
    if date.is_none() || time.is_none() {
        return Ok(None);
    }
    let date = date.unwrap().trim().to_string();
    let time = time.unwrap().trim().to_string();
    if date.is_empty() && time.is_empty() {
        Ok(None)
    } else if !date.is_empty() && time.is_empty() {
        let naive_date = NaiveDate::parse_from_str(&date, "%Y%m%d")?;
        let naive_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let dt = NaiveDateTime::new(naive_date, naive_time);
        Ok(Some(dt))
    } else if date.is_empty() && !time.is_empty() {
        Err(DcmIOError::InvalidDateTimeEmpytDate(time.to_string()))
    } else {
        let dt = NaiveDateTime::parse_from_str(&format!("{}{}", date, time), "%Y%m%d%H%M%S%.f")?;
        Ok(Some(dt))
    }
}

/// Reads and parses a combined date and time from a single element in a DICOM object.
///
/// This function retrieves the date and time from a single DICOM element specified by its tag,
/// and parses it into a [`NaiveDateTime`].
///
/// # Arguments
///
/// * `obj` - A reference to the DICOM object from which to retrieve the date and time element.
/// * `tag_date_time` - The tag of the date and time element to be retrieved.
///
/// # Returns
///
/// * `Ok(NaiveDateTime)` - The parsed date and time.
/// * `Err(DcmIOError)` - An error if the element could not be retrieved or if the parsing fails.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] if the element retrieval or parsing into `NaiveDateTime` fails.
pub(crate) fn dt_to_ndt(
    obj: &dicom_object::InMemDicomObject,
    tag_date_time: Tag,
) -> Result<NaiveDateTime, DcmIOError> {
    let date = to_string(obj, tag_date_time)?;
    let dt = NaiveDateTime::parse_from_str(&date, "%Y%m%d%H%M%S%.f")?;
    Ok(dt)
}

/// Reads and parses a date from a DICOM element into a NaiveDate.
///
/// This function retrieves the date from a DICOM object specified by its tag,
/// and parses it into a [`NaiveDate`].
///
/// # Arguments
///
/// * `obj` - A reference to the DICOM object from which to retrieve the date element.
/// * `tag` - The tag of the date element to be retrieved.
///
/// # Returns
///
/// * `Ok(NaiveDate)` - The parsed date.
/// * `Err(DcmIOError)` - An error if the date element could not be retrieved or if the parsing fails.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] if the element retrieval or parsing into `NaiveDate` fails.
pub(crate) fn to_date(
    obj: &dicom_object::InMemDicomObject,
    tag: Tag,
) -> Result<NaiveDate, DcmIOError> {
    match obj.element(tag)?.to_date()?.to_naive_date() {
        Ok(d) => Ok(d),
        Err(e) => Err(DcmIOError::InvalidDateRange(e)),
    }
}

/// Reads and parses a date from an optional DICOM element into an optional NaiveDate.
///
/// This function retrieves the date from an optional DICOM object specified by its tag,
/// and parses it into an [`Option<NaiveDate>`].
///
/// # Arguments
///
/// * `obj` - A reference to the DICOM object from which to retrieve the date element.
/// * `tag` - The tag of the date element to be retrieved.
///
/// # Returns
///
/// * `Ok(Some(NaiveDate))` - The parsed date, if the element exists and is properly parsed.
/// * `Ok(None)` - If the DICOM element does not exist.
/// * `Err(DcmIOError)` - An error if the element could not be retrieved or if the parsing fails.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] if the element retrieval or parsing into `NaiveDate` fails.
pub(crate) fn to_date_opt(
    obj: &dicom_object::InMemDicomObject,
    tag: Tag,
) -> Result<Option<NaiveDate>, DcmIOError> {
    let element = obj.element_opt(tag)?;
    if element.is_none() {
        return Ok(None);
    }
    let element = element.unwrap();
    match element.to_date()?.to_naive_date() {
        Ok(d) => Ok(Some(d)),
        Err(e) => Err(DcmIOError::InvalidDateRange(e)),
    }
}

/// Reads and parses an integer from a DICOM element.
///
/// This function retrieves the integer value from a DICOM object specified by its tag,
/// and parses it into the specified integer type `T`.
///
/// # Arguments
///
/// * `obj` - A reference to the DICOM object from which to retrieve the integer element.
/// * `tag` - The tag of the integer element to be retrieved.
///
/// # Returns
///
/// * `Ok(T)` - The parsed integer.
/// * `Err(DcmIOError)` - An error if the element could not be retrieved or if the parsing fails.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] if the element retrieval or parsing into the integer type `T` fails.
pub(crate) fn to_int<T>(obj: &dicom_object::InMemDicomObject, tag: Tag) -> Result<T, DcmIOError>
where
    T: Clone,
    T: NumCast,
    T: std::str::FromStr<Err = std::num::ParseIntError>,
{
    Ok(obj.element(tag)?.to_int()?)
}

/// Reads and parses an integer from a DICOM element into an optional value.
///
/// This function retrieves the integer value from an optional DICOM element specified by its tag,
/// and parses it into the specified integer type `T`. If the element does not exist, it returns `None`.
///
/// # Arguments
///
/// * `obj` - A reference to the DICOM object from which to retrieve the integer element.
/// * `tag` - The tag of the integer element to be retrieved.
///
/// # Returns
///
/// * `Ok(Some(T))` - The parsed integer.
/// * `Ok(None)` - If the element does not exist.
/// * `Err(DcmIOError)` - An error if the element retrieval or the parsing fails.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] if the element retrieval or parsing into the integer type `T` fails.
pub(crate) fn to_int_opt<T>(
    obj: &dicom_object::InMemDicomObject,
    tag: Tag,
) -> Result<Option<T>, DcmIOError>
where
    T: Clone,
    T: NumCast,
    T: std::str::FromStr<Err = std::num::ParseIntError>,
{
    match obj.element_opt(tag) {
        Ok(o) => match o {
            None => Ok(None),
            Some(elem) => match elem.to_int() {
                Ok(i) => Ok(Some(i)),
                Err(e) => Err(DcmIOError::from(e)),
            },
        },
        Err(e) => Err(DcmIOError::from(e)),
    }
}

/// Reads and parses multiple integers from an optional DICOM element into an optional vector.
///
/// This function attempts to retrieve multiple integer values from an optional DICOM element specified by its tag,
/// and parses them into the specified integer type `T`. If the element does not exist, it returns `None`.
///
/// # Arguments
///
/// * `obj` - A reference to the DICOM object from which to retrieve the integer elements.
/// * `tag` - The tag of the integer elements to be retrieved.
///
/// # Returns
///
/// * `Ok(Some(Vec<T>))` - A vector of parsed integers.
/// * `Ok(None)` - If the element does not exist.
/// * `Err(DcmIOError)` - An error if the element retrieval or the parsing fails.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] if the element retrieval or parsing into the integer type `T` fails.
pub(crate) fn to_ints_opt<T>(
    obj: &dicom_object::InMemDicomObject,
    tag: Tag,
) -> Result<Option<Vec<T>>, DcmIOError>
where
    T: Clone,
    T: NumCast,
    T: std::str::FromStr<Err = std::num::ParseIntError>,
{
    match obj.element_opt(tag) {
        Ok(o) => match o {
            None => Ok(None),
            Some(elem) => match elem.to_multi_int() {
                Ok(i) => Ok(Some(i)),
                Err(e) => Err(DcmIOError::from(e)),
            },
        },
        Err(e) => Err(DcmIOError::from(e)),
    }
}

/// Retrieves and parses a 32-bit floating point number from a DICOM element.
///
/// # Arguments
///
/// * `obj` - A reference to the DICOM object from which to retrieve the float element.
/// * `tag` - The tag of the float element to be retrieved.
///
/// # Returns
///
/// * `Ok(f32)` - The parsed floating point number.
/// * `Err(DcmIOError)` - An error if the element could not be retrieved or if the parsing fails.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] if the element retrieval or parsing into a `f32` fails.
pub(crate) fn to_f32(obj: &dicom_object::InMemDicomObject, tag: Tag) -> Result<f32, DcmIOError> {
    Ok(obj.element(tag)?.to_float32()?)
}

/// Retrieves and parses a 64-bit floating point number from a DICOM element.
///
/// # Arguments
///
/// * `obj` - A reference to the DICOM object from which to retrieve the float element.
/// * `tag` - The tag of the float element to be retrieved.
///
/// # Returns
///
/// * `Ok(f64)` - The parsed floating point number.
/// * `Err(DcmIOError`) - An error if the element could not be retrieved or if the parsing fails.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] if the element retrieval or parsing into a `f64` fails.
pub(crate) fn to_f64(obj: &dicom_object::InMemDicomObject, tag: Tag) -> Result<f64, DcmIOError> {
    Ok(obj.element(tag)?.to_float64()?)
}

/// Retrieves and parses an optional 64-bit floating point number from a DICOM element.
///
/// This function attempts to retrieve the float value from an optional DICOM element specified by its tag,
/// and parses it into the specified float type `f64`. If the element does not exist, it returns `None`.
///
/// # Arguments
///
/// * `obj` - A reference to the DICOM object from which to retrieve the float element.
/// * `tag` - The tag of the float element to be retrieved.
///
/// # Returns
///
/// * `Ok(Some(f64))` - The parsed floating point number.
/// * `Ok(None)` - If the element does not exist.
/// * `Err(DcmIOError)` - An error if the element retrieval or the parsing fails.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] if the element retrieval or parsing into a `f64` fails.
pub(crate) fn to_f64_opt(
    obj: &dicom_object::InMemDicomObject,
    tag: Tag,
) -> Result<Option<f64>, DcmIOError> {
    match obj.element_opt(tag) {
        Ok(o) => match o {
            None => Ok(None),
            Some(elem) => match elem.to_float64() {
                Ok(f) => Ok(Some(f)),
                Err(e) => Err(DcmIOError::from(e)),
            },
        },
        Err(e) => Err(DcmIOError::from(e)),
    }
}

/// Retrieves and parses a vector of 64-bit floating point numbers from a DICOM element.
///
/// This function attempts to retrieve the specified DICOM element by its tag and
/// parse its value(s) into a vector of `f64` numbers.
///
/// # Arguments
///
/// * `obj` - A reference to the DICOM object from which to retrieve the float elements.
/// * `tag` - The tag of the float elements to be retrieved.
///
/// # Returns
///
/// * `Ok(Vec<f64>)` - A vector containing the parsed floating point numbers.
/// * `Err(DcmIOError)` - An error if the element could not be retrieved or if the parsing fails.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] if the element retrieval or parsing into a `f64` vector fails.
pub(crate) fn to_f64s(
    obj: &dicom_object::InMemDicomObject,
    tag: Tag,
) -> Result<Vec<f64>, DcmIOError> {
    Ok(obj.element(tag)?.to_multi_float64()?)
}

/// Parses a sequence of DICOM items using a provided function, returning a vector of parsed items.
///
/// This function attempts to retrieve the DICOM element specified by its tag and parses its value,
/// which is expected to be a sequence, into a vector of type `T` by applying the given function `func`
/// to each item in the sequence.
///
/// # Arguments
///
/// * `obj` - A reference to the DICOM object from which to retrieve the sequence element.
/// * `seq_tag` - The tag of the sequence element to be retrieved.
/// * `func` - A function that takes a reference to a DICOM object and returns a result of type `T`.
///
/// # Returns
///
/// * `Ok(Vec<T>)` - A vector containing the parsed items.
/// * `Err(DcmIOError)` - An error if the element could not be retrieved, if it is not a sequence,
///   or if the parsing of one of its items fails.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] in the following cases:
/// * The retrieval of the element fails.
/// * The element is not a sequence.
/// * One of the items in the sequence cannot be parsed.
pub(crate) fn from_seq<T, F>(
    obj: &dicom_object::InMemDicomObject,
    seq_tag: Tag,
    func: F,
) -> Result<Vec<T>, DcmIOError>
where
    F: Fn(&dicom_object::InMemDicomObject) -> Result<T, DcmIOError>,
{
    let seq = obj.element(seq_tag)?;
    let mut v = Vec::new();
    match seq.value() {
        Value::Primitive(_) => {
            return Err(DcmIOError::ElementIsNotSequence(seq_tag));
        }
        Value::Sequence(sq) => {
            for item in sq.items() {
                match func(item) {
                    Ok(t) => {
                        v.push(t);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Value::PixelSequence(_) => {
            return Err(DcmIOError::PixelSequenceNotSupported(seq_tag));
        }
    }
    Ok(v)
}

/// Parses an optional sequence of DICOM items using a provided function,
/// returning an optional vector of parsed items.
///
/// This function attempts to retrieve the DICOM element specified by its tag and parses its value,
/// which is expected to be a sequence, into an optional vector of type `T` by applying the given function `func`
/// to each item in the sequence. If the element is not found, it returns `Ok(None)`.
///
/// # Arguments
///
/// * `obj` - A reference to the DICOM object from which to retrieve the sequence element.
/// * `seq_tag` - The tag of the sequence element to be retrieved.
/// * `func` - A function that takes a reference to a DICOM object and returns a result of type `T`.
///
/// # Returns
///
/// * `Ok(Some(Vec<T>))` - A vector containing the parsed items if the element is found and successfully parsed.
/// * `Ok(None)` - If the tag is not found in the DICOM object.
/// * `Err(DcmIOError)` - An error if the element retrieval fails, if it is not a sequence,
///   or if the parsing of one of its items fails.
///
/// # Errors
///
/// This function returns a [`DcmIOError`] in the following cases:
/// * The retrieval of the element fails.
/// * The element is not a sequence.
/// * One of the items in the sequence cannot be parsed.
pub(crate) fn from_seq_opt<T, F>(
    obj: &dicom_object::InMemDicomObject,
    seq_tag: Tag,
    func: F,
) -> Result<Option<Vec<T>>, DcmIOError>
where
    F: Fn(&dicom_object::InMemDicomObject) -> Result<T, DcmIOError>,
{
    let seq = obj.element_opt(seq_tag)?;
    if seq.is_none() {
        return Ok(None);
    }
    let v = from_seq(obj, seq_tag, func)?;
    Ok(Some(v))
}

#[cfg(test)]
mod test {
    use crate::io::{da_tm_to_ndt, da_tm_to_ndt_opt, to_ints_opt, DcmIOError};
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use dicom_core::smallvec::smallvec;
    use dicom_core::{DataElement, PrimitiveValue, VR};
    use dicom_dictionary_std::tags::{
        ACQUISITION_DATE_TIME, COLUMNS, CONTENT_DATE, CONTENT_TIME, DEVICE_DIAMETER, KVP,
        PATIENT_ID, PATIENT_NAME, ROI_DISPLAY_COLOR, ROWS, SERIES_NUMBER, SLICE_LOCATION,
        STUDY_DATE, STUDY_TIME,
    };
    use dicom_object::InMemDicomObject;
    use log::{debug, LevelFilter};

    fn init_logger() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(LevelFilter::Trace)
            .try_init();
    }

    fn get_test_data() -> InMemDicomObject {
        let mut obj = InMemDicomObject::new_empty();
        obj.put(DataElement::new(PATIENT_ID, dicom_core::VR::LO, "X01"));
        obj.put(DataElement::new(STUDY_DATE, dicom_core::VR::DA, "20240717"));
        obj.put(DataElement::new(
            STUDY_TIME,
            dicom_core::VR::TM,
            "100601.004858",
        ));
        obj.put(DataElement::new(
            ACQUISITION_DATE_TIME,
            dicom_core::VR::DT,
            "20240717100457.868000",
        ));
        obj.put(DataElement::new(KVP, dicom_core::VR::DS, "120.5"));
        obj.put(DataElement::new(ROWS, dicom_core::VR::US, "512"));
        obj.put(DataElement::new(COLUMNS, dicom_core::VR::US, "256"));
        obj.put(DataElement::new(SLICE_LOCATION, dicom_core::VR::DS, ""));
        obj
    }

    #[test]
    fn test_to_string_valid() {
        let obj = get_test_data();
        let tag = PATIENT_ID;

        let result = super::to_string(&obj, tag);
        assert!(result.is_ok());

        let expected = "X01".to_string();
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_to_string_opt_valid_input() {
        let obj = get_test_data();
        let tag = PATIENT_ID;

        let result = super::to_string_opt(&obj, tag);
        assert!(result.is_ok());

        let o = result.unwrap();
        assert!(o.is_some());
        let expected = "X01".to_string();
        assert_eq!(o.unwrap(), expected);
    }

    #[test]
    fn test_to_string_opt_missing_input() {
        let obj = get_test_data();
        let tag = PATIENT_NAME; // Assuming PATIENT_NAME is not present in the test data

        let result = super::to_string_opt(&obj, tag);
        assert!(result.is_ok());

        let o = result.unwrap();
        assert!(o.is_none());
    }

    #[test]
    fn test_read_tm_to_ndt_valid_input() {
        let obj = get_test_data();
        let tag_date = STUDY_DATE;
        let tag_time = STUDY_TIME;

        let result = da_tm_to_ndt(&obj, tag_date, tag_time);
        assert!(result.is_ok());

        let date = NaiveDate::from_ymd_opt(2024, 7, 17).unwrap();
        let time = NaiveTime::from_hms_micro_opt(10, 6, 1, 4858).unwrap();

        // let expected = NaiveDateTime::parse_from_str("20240717100601.004858", "%Y%m%d%H%M%S%.f").unwrap();
        let expected = NaiveDateTime::new(date, time);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_read_tm_to_ndt_invalid_input() {
        let mut obj = get_test_data();
        obj.put(DataElement::new(
            CONTENT_DATE,
            dicom_core::VR::DA,
            "20240717",
        ));
        obj.put(DataElement::new(STUDY_TIME, dicom_core::VR::TM, "100601"));
        let tag_date = CONTENT_DATE;
        let tag_time = CONTENT_TIME;

        let result = da_tm_to_ndt(&obj, tag_date, tag_time);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_tm_to_ndt_opt_valid_input() {
        let obj = get_test_data();
        let tag_date = STUDY_DATE;
        let tag_time = STUDY_TIME;

        let result = da_tm_to_ndt_opt(&obj, tag_date, tag_time);
        assert!(result.is_ok());

        let date = NaiveDate::from_ymd_opt(2024, 7, 17).unwrap();
        let time = NaiveTime::from_hms_micro_opt(10, 6, 1, 4858).unwrap();

        // let expected = NaiveDateTime::parse_from_str("20240717100601.004858", "%Y%m%d%H%M%S%.f").unwrap();
        let expected = NaiveDateTime::new(date, time);
        let o = result.unwrap();
        assert!(o.is_some());
        assert_eq!(o.unwrap(), expected);
    }

    #[test]
    fn test_read_tm_to_ndt_opt_invalid_input() {
        let mut obj = get_test_data();
        obj.put(DataElement::new(CONTENT_DATE, dicom_core::VR::DA, "202407"));
        obj.put(DataElement::new(STUDY_TIME, dicom_core::VR::TM, "100601"));
        let tag_date = CONTENT_DATE;
        let tag_time = CONTENT_TIME;

        let result = da_tm_to_ndt_opt(&obj, tag_date, tag_time);
        assert!(result.is_ok());
        let o = result.unwrap();
        assert!(o.is_none());
    }

    #[test]
    fn test_read_tm_to_ndt_opt_missing_input() {
        let obj = get_test_data();
        let tag_date = CONTENT_DATE;
        let tag_time = CONTENT_TIME;

        let result = da_tm_to_ndt_opt(&obj, tag_date, tag_time);
        assert!(result.is_ok());
        let o = result.unwrap();
        assert!(o.is_none());
    }

    #[test]
    fn test_read_dt_to_ndt_valid_input() {
        let obj = get_test_data();
        let tag = ACQUISITION_DATE_TIME;

        let result = super::dt_to_ndt(&obj, tag);
        assert!(result.is_ok());

        let expected =
            NaiveDateTime::parse_from_str("20240717100457.868000", "%Y%m%d%H%M%S%.f").unwrap();
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_read_dt_to_ndt_invalid_input() {
        let mut obj = get_test_data();
        let tag = ACQUISITION_DATE_TIME;

        // Modify the test data with an invalid date-time string
        obj.put(DataElement::new(
            tag,
            dicom_core::VR::DT,
            "invalid_date_time",
        ));

        let result = super::dt_to_ndt(&obj, tag);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_date_valid() {
        let obj = get_test_data();
        let tag = STUDY_DATE;

        let result = super::to_date(&obj, tag);
        assert!(result.is_ok());

        let expected = NaiveDate::parse_from_str("20240717", "%Y%m%d").unwrap();
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_to_f32_valid() {
        let obj = get_test_data();
        let tag = KVP;

        let result = super::to_f32(&obj, tag);
        assert!(result.is_ok());

        let expected = 120.5_f32;
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_to_f32_invalid() {
        let obj = get_test_data();
        let tag = PATIENT_NAME;

        let result = super::to_f32(&obj, tag);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_f64_valid() {
        let obj = get_test_data();
        let tag = KVP;

        let result = super::to_f64(&obj, tag);
        assert!(result.is_ok());

        let expected = 120.5_f64;
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_to_f64_invalid() {
        let obj = get_test_data();
        let tag = PATIENT_NAME;

        let result = super::to_f64(&obj, tag);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_f64_opt_valid() {
        let obj = get_test_data();
        let tag = KVP;

        let result = super::to_f64_opt(&obj, tag);
        assert!(result.is_ok());

        let expected = Some(120.5_f64);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_to_f64_opt_invalid() {
        init_logger();
        let obj = get_test_data();
        let tag = PATIENT_NAME;

        let result = super::to_f64_opt(&obj, tag);
        assert!(result.is_ok());
    }

    #[test]
    fn test_to_f64_opt_missing() {
        let obj = get_test_data();
        let tag = DEVICE_DIAMETER; // Use an appropriate tag that is known to be missing in get_test_data()

        let result = super::to_f64_opt(&obj, tag);
        assert!(result.is_ok());
        let o = result.unwrap();
        assert!(o.is_none());
    }

    #[test]
    fn test_to_int_opt_valid() {
        let obj = get_test_data(); // Reusing existing method to create a test object
        let tag = SERIES_NUMBER; // Assuming KVP tag stores valid integer data

        let result = super::to_int_opt::<i32>(&obj, tag);
        assert!(result.is_ok());
        let o = result.unwrap();
        assert!(o.is_none());
    }

    #[test]
    fn test_to_int_opt_valid_non_existent() {
        let obj = get_test_data(); // Reusing existing method to create a test object
        let tag = ROWS; // Assuming KVP tag stores valid integer data

        let result = super::to_int_opt::<i32>(&obj, tag);
        assert!(result.is_ok());

        let expected = Some(512);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_to_int_opt_invalid() {
        let obj = get_test_data(); // Reusing existing method to create a test object
        let tag = PATIENT_NAME; // Assuming PATIENT_NAME tag stores a string that is not an integer

        let result = super::to_int_opt::<i32>(&obj, tag);
        assert!(result.is_ok());
    }
    #[test]
    fn test_to_ints_opt_some() {
        let mut obj = InMemDicomObject::new_empty();
        let tag = ROI_DISPLAY_COLOR;
        let values = smallvec![255, 18, 125];

        let elem = DataElement::new(tag, VR::IS, PrimitiveValue::U8(values));
        obj.put(elem);

        let result: Result<Option<Vec<u8>>, DcmIOError> = to_ints_opt(&obj, tag);
        if let Err(e) = &result {
            debug!("{:?}", e);
            assert!(false);
        }
        assert!(result.is_ok());
        let values = result.unwrap();
        assert!(values.is_some());
        let values = values.unwrap();
        assert_eq!(values, vec![255, 18, 125]);
    }

    #[test]
    fn test_to_ints_opt_none() {
        let obj = InMemDicomObject::new_empty();
        let tag = ROI_DISPLAY_COLOR;

        let result: Result<Option<Vec<i8>>, DcmIOError> = to_ints_opt(&obj, tag);
        assert!(result.is_ok());
        let values = result.unwrap();
        assert!(values.is_none());
    }

    #[test]
    fn test_to_ints_opt_invalid_int() {
        let mut obj = InMemDicomObject::new_empty();
        let tag = ROI_DISPLAY_COLOR;
        let elem = DataElement::new(tag, VR::PN, "X_Head^Rando");
        obj.put(elem);

        let result: Result<Option<Vec<u8>>, DcmIOError> = to_ints_opt(&obj, tag);
        assert!(result.is_err());
    }
}
