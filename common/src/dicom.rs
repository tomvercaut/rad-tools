use dicom_object::{DefaultDicomObject, OpenFileOptions, ReadError, Tag};
use tracing::debug;

/// Reads a DICOM file from the specified path.
///
/// # Arguments
///
/// * `path` - The path to the DICOM file to read
///
/// # Returns
///
/// * `Ok(DefaultDicomObject)` - The DICOM object containing the file's metadata
/// * `Err(ReadError)` - If an error occurs while reading the DICOM file//
pub fn open_file<P>(path: P) -> Result<DefaultDicomObject, ReadError>
where
    P: AsRef<std::path::Path>,
{
    let path = path.as_ref();
    debug!("Read DICOM data from: {path:#?}");
    OpenFileOptions::new().open_file(path)
}

/// Reads a DICOM file from the specified path until a specific tag is encountered.
///
/// # Arguments
///
/// * `path` - The path to the DICOM file to read
/// * `tag` - The DICOM tag at which to stop reading
///
/// # Returns
///
/// * `Ok(DefaultDicomObject)` - The DICOM object containing the file's metadata
/// * `Err(ReadError)` - If an error occurs while reading the DICOM file
pub fn open_file_until<P>(path: P, tag: Tag) -> Result<DefaultDicomObject, ReadError>
where
    P: AsRef<std::path::Path>,
{
    let path = path.as_ref();
    debug!("Read DICOM data from: {path:#?}");
    OpenFileOptions::new().read_until(tag).open_file(path)
}

/// Reads a DICOM file from the provided reader.
///
/// # Arguments
///
/// * `reader` - The reader implementing std::io::Read trait to read DICOM data from
///
/// # Returns
///
/// * `Ok(DefaultDicomObject)` - The DICOM object containing the file's metadata
/// * `Err(ReadError)` - If an error occurs while reading the DICOM file
pub fn from_reader<R>(reader: R) -> Result<DefaultDicomObject, ReadError>
where
    R: std::io::Read,
{
    debug!("Read DICOM data from reader");
    OpenFileOptions::new().from_reader(reader)
}

/// Reads a DICOM file from the provided reader until a specific tag is encountered.
///
/// # Arguments
///
/// * `reader` - The reader implementing std::io::Read trait to read DICOM data from
/// * `tag` - The DICOM tag at which to stop reading
///
/// # Returns
///
/// * `Ok(DefaultDicomObject)` - The DICOM object containing the file's metadata
/// * `Err(ReadError)` - If an error occurs while reading the DICOM file
pub fn from_reader_until<R>(reader: R, tag: Tag) -> Result<DefaultDicomObject, ReadError>
where
    R: std::io::Read,
{
    debug!("Read DICOM data from reader");
    OpenFileOptions::new().read_until(tag).from_reader(reader)
}
