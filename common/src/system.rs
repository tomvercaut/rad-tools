use std::path::{Path, PathBuf};
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Required executable not found in PATH")]
    ExecutableNotFound,
    #[error("")]
    IO(#[from] std::io::Error),
    #[error("Unable to find a unique filename")]
    UnableToFindAUniqueFilename,
}

pub type Result<T> = std::result::Result<T, Error>;

/// Checks if an executable exists in the system's PATH environment variable.
///
/// This function uses the native command-line tools (`which` on Unix-like systems,
/// `where` on Windows) to determine if the specified executable is available in PATH.
///
/// # Arguments
///
/// * `executable` - The name of the executable to search for
///
/// # Returns
///
/// * `Ok(())` if the executable is found in PATH
/// * `Err(Error::ExecutableNotFound)` if the executable is not found
///
/// # Examples
///
/// ```
/// use rad_tools_common::system::which;
///
/// // Check if 'ls' exists in PATH
/// match which("ls") {
///     Ok(_) => println!("ls is available"),
///     Err(_) => println!("ls not found in PATH"),
/// }
/// ```
pub fn which<S>(executable: S) -> Result<()>
where
    S: AsRef<str>,
{
    let executable = executable.as_ref();
    // Check if an executable exists in PATH
    let which_cmd = if cfg!(windows) { "where" } else { "which" };
    if !std::process::Command::new(which_cmd)
        .arg(executable)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        error!("{executable:#?} executable not found in PATH");
        return Err(Error::ExecutableNotFound);
    }
    Ok(())
}

pub trait UniqueFilenameGenerator {
    /// Generates a unique filename.
    ///
    /// # Returns
    ///
    /// * `Ok(PathBuf)` - A path to a unique filename that doesn't exist yet
    /// * `Err` - If unable to generate a unique filename
    fn create_unique_filename(&self) -> Result<PathBuf>;
}

pub struct UniqueFilenameGeneratorByIndexSuffix {
    dir: PathBuf,
    basename: String,
    extension: String,
    attempts: usize,
}

impl UniqueFilenameGeneratorByIndexSuffix {
    pub fn new<P>(dir: P, basename: String, extension: String, attempts: usize) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            dir: dir.as_ref().to_path_buf(),
            basename,
            extension,
            attempts: match attempts {
                0 => usize::MAX,
                _ => attempts,
            },
        }
    }
}

impl UniqueFilenameGenerator for UniqueFilenameGeneratorByIndexSuffix {
    fn create_unique_filename(&self) -> Result<PathBuf> {
        for i in 0..self.attempts {
            let filename = format!("{}_{}.{}", self.basename, i, self.extension);
            let path = self.dir.join(filename);
            if !path.exists() {
                return Ok(path);
            }
        }
        Err(Error::UnableToFindAUniqueFilename)
    }
}
