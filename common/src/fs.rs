use std::ffi::{OsStr, OsString};
use std::io::Read;
use std::path::PathBuf;

/// Trait for types that can provide a unique file path.
///
/// This trait should be implemented by types that need to generate
/// unique file paths based on their internal data.
pub trait UniquePathGenerator {
    type UniquePathError;

    /// Returns a unique path for this object.
    ///
    /// # Returns
    ///
    /// * `PathBuf` - A unique path based on the object's properties
    fn get_unique_path(&self) -> Result<PathBuf, Self::UniquePathError>;
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum DefaultUniquePathError {
    #[error("Input directory does not exists")]
    DirNotExists,
    #[error("No unique file path found after reaching {0} attemps.")]
    LimitReached(usize),
}

/// A default implementation of the UniquePathGenerator trait that generates unique file paths
/// by appending numbers to the base filename if a file already exists.
///
/// # Example
///
/// ```
/// use std::ffi::{OsStr};
/// use std::fs::File;
/// use std::io::Write;
/// use std::path::PathBuf;
/// use rad_tools_common::fs::{DefaultUniquePathError, DefaultUniquePathGenerator, UniquePathGenerator};
///
///let path = std::env::temp_dir().join("rad-tools-common");
///        if path.exists() {
///            std::fs::remove_dir_all(&path).unwrap();
///        }
///        std::fs::create_dir_all(&path).unwrap();
///
///        let generator = DefaultUniquePathGenerator {
///            dir: path.clone(),
///            name: OsStr::new("test"),
///            extension: Some(OsStr::new("txt")),
///            limit: 1
///        };
///
///        let test_path = generator.get_unique_path().unwrap();
///        assert_eq!(test_path.file_name().unwrap().to_str().unwrap(), "test.txt");
///        {
///            let mut file = File::create_new(test_path).unwrap();
///            file.write_all(b"test").unwrap();
///        }
///
///        let test_path = generator.get_unique_path().unwrap();
///        assert_eq!(test_path.file_name().unwrap().to_str().unwrap(), "test_0.txt");
///        {
///            let mut file = File::create_new(test_path).unwrap();
///            file.write_all(b"test_0").unwrap();
///        }
///
///        let r = generator.get_unique_path();
///        assert_eq!(r.err().unwrap(), DefaultUniquePathError::LimitReached(1));
///
///        std::fs::remove_dir_all(&path).unwrap();
/// ```
///
/// If "/tmp/test.txt" exists, it will try "/tmp/test_0.txt", "/tmp/test_1.txt", etc.
/// until either a unique path is found or the limit is reached.
#[derive(Clone, Debug)]
pub struct DefaultUniquePathGenerator<'a> {
    /// The directory in which to generate the unique file path.
    pub dir: PathBuf,
    /// Base name of the file without extension.
    /// When generating unique paths, numbers will be appended to this name if needed.
    pub name: &'a OsStr,
    /// Optional file extension without the leading dot.
    /// If not provided, the file will be created without an extension.
    pub extension: Option<&'a OsStr>,
    /// Maximum number of attempts to generate a unique path by appending numbers.
    /// If this limit is reached without finding a unique path, an error will be returned.
    pub limit: usize,
}

impl<'a> UniquePathGenerator for DefaultUniquePathGenerator<'a> {
    type UniquePathError = DefaultUniquePathError;

    fn get_unique_path(&self) -> Result<PathBuf, Self::UniquePathError> {
        let path = self.dir.clone();
        if !path.is_dir() {
            return Err(DefaultUniquePathError::DirNotExists);
        }
        let os_ext = match self.extension.as_ref() {
            None => OsString::new(),
            Some(ext) => {
                if ext.is_empty() {
                    OsString::new()
                } else {
                    let mut tos = OsString::from(".");
                    tos.push(ext);
                    tos
                }
            }
        };
        let tpath = match self.extension.as_ref() {
            None => path.join(self.name),
            Some(_) => {
                let mut tos = OsString::from(self.name);
                tos.push(&os_ext);
                path.join(tos)
            }
        };
        if !tpath.exists() {
            return Ok(tpath);
        }
        let mut i = 0;
        while i < self.limit {
            let mut tos = OsString::from(self.name);
            tos.push(format!("_{}", i));
            let tpath = match self.extension.as_ref() {
                None => path.join(tos),
                Some(_) => {
                    tos.push(&os_ext);
                    path.join(tos)
                }
            };
            if !tpath.exists() {
                return Ok(tpath);
            }
            i += 1;
        }
        Err(DefaultUniquePathError::LimitReached(i))
    }
}

/// Compare two std::io::Read instances for binary equality using a buffered read approach.
pub fn binary_eq(f1: &mut impl Read, f2: &mut impl Read) -> Result<bool, std::io::Error> {
    const BUF_SIZE: usize = 1024;
    let mut buf1 = [0u8; BUF_SIZE];
    let mut buf2 = [0u8; BUF_SIZE];
    loop {
        let n1 = f1.read(&mut buf1)?;
        let n2 = f2.read(&mut buf2)?;
        if n1 != n2 || buf1[..n1] != buf2[..n2] {
            return Ok(false);
        }
        if n1 == 0 {
            return Ok(true);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    fn generate_test_data() -> [u8; 4096] {
        // std::fs::File::create("tests/test_files/test.txt").unwrap()
        let mut a = [0u8; 4096];
        for i in 0..4096 {
            a[i] = (i % u8::MAX as usize) as u8;
        }
        a
    }

    #[test]
    fn test_binary_eq() {
        let d1 = generate_test_data();
        let d2 = generate_test_data();
        let mut c1 = Cursor::new(d1);
        let mut c2 = Cursor::new(d2);
        assert!(binary_eq(&mut c1, &mut c2).unwrap());
    }

    #[test]
    fn test_binary_eq_d1_modified_byte() {
        let mut d1 = generate_test_data();
        let d2 = generate_test_data();
        d1[386] = 0;
        let mut c1 = Cursor::new(d1);
        let mut c2 = Cursor::new(d2);
        assert!(!binary_eq(&mut c1, &mut c2).unwrap());
    }

    #[test]
    fn test_binary_eq_d2_modified_byte() {
        let d1 = generate_test_data();
        let mut d2 = generate_test_data();
        d2[386] = 0;
        let mut c1 = Cursor::new(d1);
        let mut c2 = Cursor::new(d2);
        assert!(!binary_eq(&mut c1, &mut c2).unwrap());
    }

    #[test]
    fn test_binary_eq_length_mismatch_d1() {
        let d1 = generate_test_data();
        let d2 = generate_test_data();
        let mut c1 = Cursor::new(&d1[0..4095]);
        let mut c2 = Cursor::new(&d2);
        assert!(!binary_eq(&mut c1, &mut c2).unwrap());
    }

    #[test]
    fn test_binary_eq_length_mismatch_d2() {
        let d1 = generate_test_data();
        let d2 = generate_test_data();
        let mut c1 = Cursor::new(d1);
        let mut c2 = Cursor::new(&d2[0..4095]);
        assert!(!binary_eq(&mut c1, &mut c2).unwrap());
    }
}