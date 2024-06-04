use crate::DosimetryToolsError;
use crate::DosimetryToolsError::{InvalidBytesF32, InvalidBytesF64, InvalidBytesU16};
use base64::Engine;
use quick_xml::events::{BytesCData, BytesText};

/// Converts a base64-encoded string of `f32` values to a vector of `f64` values.
///
/// # Arguments
///
/// * `s` - The base64-encoded string of `f32` values.
///
/// # Returns
///
/// Returns a `Result` that contains a vector of `f64` values on success, or an `DosimetryToolsError` on error.
///
/// # Errors
///
/// Returns an `InvalidBytesF32` error if the number of bytes is not divisible by 4.
///
/// # Examples
///
/// ```
/// use peeteewee::DosimetryToolsError;
/// use peeteewee::decode::base64_f32s_as_f64s;
///
/// let result = base64_f32s_as_f64s("AbCdEfGh".to_string());
/// match result {
///     Ok(values) => println!("Success: {:?}", values),
///     Err(err) => match err {
///         DosimetryToolsError::InvalidBytesF32(n) => println!("Invalid number of bytes: {}", n),
///         _ => {}
///     },
/// }
/// ```
pub fn base64_f32s_as_f64s(s: String) -> Result<Vec<f64>, DosimetryToolsError> {
    let bytes = base64::engine::general_purpose::STANDARD.decode(s.as_str())?;
    let n = bytes.len();
    if n % 4 != 0 {
        return Err(InvalidBytesF32(n));
    }
    let mut v = Vec::with_capacity(n / 4);
    let mut i = 0;
    while i < n {
        let buf: [u8; 4] = [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]];
        let f = f32::from_le_bytes(buf);
        v.push(f as f64);
        i += 4;
    }
    Ok(v)
}

/// Convert a base64-encoded string into a vector of f64 values.
///
/// # Arguments
///
/// * `s` - A base64-encoded string.
///
/// # Returns
///
/// * `Ok(Vec<f64>)` - A vector of f64 values if the conversion is successful.
///
/// * `Err(DosimetryToolsError)` - An error if the conversion fails.
///
/// # Examples
///
/// ```
/// use peeteewee::DosimetryToolsError;
/// use peeteewee::decode::base64_f64s;
///
/// let result = base64_f64s("SGVsbG8gd29ybGQ=".to_string());
/// assert_eq!(result.unwrap(), vec![72.0, 101.0, 108.0, 108.0, 111.0, 32.0, 119.0, 111.0, 114.0, 108.0, 100.0]);
/// ```
///
/// # Errors
///
/// This function will return an error under the following conditions:
///
/// * If the length of the input string is not a multiple of 8 bytes, an `InvalidBytesF64` error is returned.
pub fn base64_f64s(s: String) -> Result<Vec<f64>, DosimetryToolsError> {
    let bytes = base64::engine::general_purpose::STANDARD.decode(s.as_str())?;
    let n = bytes.len();
    let n_bytes = 8;
    if n % n_bytes != 0 {
        return Err(InvalidBytesF64(n));
    }
    let mut v = Vec::with_capacity(n / n_bytes);
    let mut i = 0;
    while i < n {
        let buf: [u8; 8] = [
            bytes[i],
            bytes[i + 1],
            bytes[i + 2],
            bytes[i + 3],
            bytes[i + 4],
            bytes[i + 5],
            bytes[i + 6],
            bytes[i + 7],
        ];
        let f = f64::from_le_bytes(buf);
        v.push(f);
        i += n_bytes;
    }
    Ok(v)
}

/// Converts a base64 encoded string into a vector of u64 values.
///
/// # Arguments
///
/// * `s` - A base64 encoded string to be converted.
///
/// # Return
///
/// Returns `Ok` with a vector of u64 values if the conversion is successful.
/// Returns `Err` with a `DosimetryToolsError` if the conversion fails.
///
/// # Examples
///
/// ```
/// use peeteewee::decode::base64_to_u64s;
///
/// let result = base64_to_u64s("SGVsbG8gd29ybGQh".to_string());
/// assert_eq!(result.unwrap(), vec![72623859790382856]);
/// ```
///
/// # Errors
///
/// The function will return `Err` for the following reasons:
///
/// * If the input string is not a valid base64 encoding.
/// * If the length of the decoded bytes is not a multiple of 8.
///
/// # Panics
///
/// The function will panic if the input string contains invalid UTF-8 characters.
pub fn base64_to_u64s(s: String) -> Result<Vec<u64>, DosimetryToolsError> {
    let bytes = base64::engine::general_purpose::STANDARD.decode(s.as_str())?;
    let n = bytes.len();
    let n_bytes = 8;
    if n % n_bytes != 0 {
        return Err(InvalidBytesF64(n));
    }
    let mut v = Vec::with_capacity(n / n_bytes);
    let mut i = 0;
    while i < n {
        let buf: [u8; 8] = [
            bytes[i],
            bytes[i + 1],
            bytes[i + 2],
            bytes[i + 3],
            bytes[i + 4],
            bytes[i + 5],
            bytes[i + 6],
            bytes[i + 7],
        ];
        let f = u64::from_le_bytes(buf);
        v.push(f);
        i += n_bytes;
    }
    Ok(v)
}

/// Decodes a base64 string into a vector of u16 values.
///
/// # Arguments
///
/// * `s` - A base64 encoded string.
///
/// # Returns
///
/// Returns a `Result` containing a vector of u16 values if the decoding is successful,
/// or a `DosimetryToolsError` if an error occurs during decoding.
///
/// # Errors
///
/// An error is returned if the length of the decoded bytes is not divisible by 2.
///
/// # Examples
///
/// ```
/// use peeteewee::decode::base64_u16s;
///
/// let encoded = "AQABDA=="; // Example base64 string
///
/// match base64_u16s(encoded.to_string()) {
///     Ok(decoded) => {
///         // Use the decoded u16 values
///         for value in decoded {
///             println!("{}", value);
///         }
///     }
///     Err(error) => {
///         // Handle the error
///         println!("Decoding error: {:?}", error);
///     }
/// }
/// ```
pub fn base64_u16s(s: String) -> Result<Vec<u16>, DosimetryToolsError> {
    let bytes = base64::engine::general_purpose::STANDARD.decode(s.as_str())?;
    let n = bytes.len();
    let n_bytes = 2;
    if n % n_bytes != 0 {
        return Err(InvalidBytesU16(n));
    }
    let mut v = Vec::with_capacity(n / n_bytes);
    let mut i = 0;
    while i < n {
        let buf: [u8; 2] = [bytes[i], bytes[i + 1]];
        let f = u16::from_le_bytes(buf);
        v.push(f);
        i += n_bytes;
    }
    Ok(v)
}

pub fn to_f64(s: String) -> Result<f64, DosimetryToolsError> {
    s.parse::<f64>()
        .map_err(DosimetryToolsError::ParseFloatError)
}

pub fn to_u32(s: String) -> Result<u32, DosimetryToolsError> {
    s.parse::<u32>().map_err(DosimetryToolsError::ParseIntError)
}

pub fn byte_text_to_string(text: &BytesText) -> Result<String, DosimetryToolsError> {
    let tmp = std::str::from_utf8(text.as_ref())?;
    let text = tmp.to_string();
    Ok(text)
}

pub fn byte_cdata_to_string(text: &BytesCData) -> Result<String, DosimetryToolsError> {
    let tmp = std::str::from_utf8(text.as_ref())?;
    let text = tmp.to_string();
    Ok(text)
}
