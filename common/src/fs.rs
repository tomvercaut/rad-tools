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
///            name: "test".to_string(),
///            extension: Some("txt".to_string()),
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
pub struct DefaultUniquePathGenerator {
    /// The directory in which to generate the unique file path.
    pub dir: PathBuf,
    /// Base name of the file without extension.
    /// When generating unique paths, numbers will be appended to this name if needed.
    pub name: String,
    /// Optional file extension without the leading dot.
    /// If not provided, the file will be created without an extension.
    pub extension: Option<String>,
    /// Maximum number of attempts to generate a unique path by appending numbers.
    /// If this limit is reached without finding a unique path, an error will be returned.
    pub limit: usize,
}

impl UniquePathGenerator for DefaultUniquePathGenerator {
    type UniquePathError = DefaultUniquePathError;

    fn get_unique_path(&self) -> Result<PathBuf, Self::UniquePathError> {
        let path = self.dir.clone();
        if !path.is_dir() {
            return Err(DefaultUniquePathError::DirNotExists);
        }
        let tpath = match self.extension {
            None => path.join(&self.name),
            Some(_) => path.join(format!(
                "{}.{}",
                self.name,
                self.extension.as_ref().unwrap()
            )),
        };
        if !tpath.exists() {
            return Ok(tpath);
        }
        let mut i = 0;
        while i < self.limit {
            let tpath = match self.extension {
                None => path.join(format!("{}_{}", self.name, i)),
                Some(_) => path.join(format!(
                    "{}_{}.{}",
                    self.name,
                    i,
                    self.extension.as_ref().unwrap()
                )),
            };
            if !tpath.exists() {
                return Ok(tpath);
            }
            i += 1;
        }
        Err(DefaultUniquePathError::LimitReached(i))
    }
}
