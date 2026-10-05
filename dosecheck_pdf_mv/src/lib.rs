use clap::Parser;
use std::path::{Path, PathBuf};
use tracing::debug;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Input directory does not exist")]
    InputDirNotExist,
    #[error("Output directory does not exist")]
    OutputDirNotExist,
    #[error("Watch interval must be a positive integer")]
    InvalidWatchInterval,
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Move dosecheck PDFs from one directory to another.
///
#[derive(Parser, Debug, Clone)]
#[command(version)]
pub struct Args {
    #[arg(short, long, value_name = "INPUT_DIR", help = "Input directory")]
    pub input: PathBuf,
    #[arg(short, long, value_name = "OUTPUT_DIR", help = "Output directory")]
    pub output: PathBuf,
    #[arg(
        short,
        long,
        value_name = "MILLISECONDS",
        help = "Watch for changes in the input directory (watch interval in milliseconds)."
    )]
    pub watch: Option<u64>,
}

impl Args {
    pub fn validate(&self) -> Result<()> {
        if !self.input.is_dir() {
            return Err(Error::InputDirNotExist);
        }
        if !self.output.is_dir() {
            return Err(Error::OutputDirNotExist);
        }
        if let Some(interval) = self.watch
            && interval == 0
        {
            return Err(Error::InvalidWatchInterval);
        }
        Ok(())
    }
}

pub fn mv_dosecheck_pdfs_watch<P1, P2>(
    input: P1,
    output: P2,
    interval_ms: Option<u64>,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let input = input.as_ref();
    let output = output.as_ref();

    debug!("Moving dosecheck PDFs from {:#?} to {:#?}", input, output);

    match interval_ms {
        None => mv_dosecheck_pdfs(input, output),
        Some(millisec) => {
            let dur = std::time::Duration::from_millis(millisec);
            loop {
                mv_dosecheck_pdfs(input, output)?;
                std::thread::sleep(dur);
            }
        }
    }
}

/// Moves DoseCHECK PDF files from the input directory to the output directory.
///
/// Iterates (non-recursively) over entries in the `input` directory, identifies regular files that
/// have a `.pdf` extension and whose filename begins with `"DoseCHECK_"`, and moves
/// (renames) them into the `output` directory, preserving their original filenames.
///
/// # Arguments
///
/// * `input` - The path to the source directory containing the files to inspect.
/// * `output` - The path to the target directory where matching PDF files will be moved.
///
/// # Errors
///
/// Returns an [`anyhow::Error`] if reading the input directory fails, reading entry metadata
/// fails, or moving any file fails (e.g. due to permission issues).
pub fn mv_dosecheck_pdfs<P1, P2>(input: P1, output: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let input = input.as_ref();
    let output = output.as_ref();

    for entry in std::fs::read_dir(input)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        if !meta.is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("pdf") {
            continue;
        }
        let filename = path.file_name().unwrap().to_str().unwrap();
        if !filename.starts_with("DoseCHECK_") {
            continue;
        }
        let output_path = output.join(filename);
        // Copying the file instead of moving it, this should prevent problems with cross-device moves.
        debug!("Copying file: {:?} to {:?}", path, output_path);
        std::fs::copy(&path, output_path)?;
        debug!("Removing file: {:?}", path);
        std::fs::remove_file(path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_mv_dosecheck_pdfs_basic() -> anyhow::Result<()> {
        let input_dir = tempdir()?;
        let output_dir = tempdir()?;

        let file_name = "DoseCHECK_patient_123.pdf";
        let input_file = input_dir.path().join(file_name);
        let output_file = output_dir.path().join(file_name);

        let content = b"Sample PDF Content for DoseCHECK";
        fs::write(&input_file, content)?;

        assert!(input_file.exists());
        assert!(!output_file.exists());

        mv_dosecheck_pdfs(input_dir.path(), output_dir.path())?;

        assert!(!input_file.exists(), "Input file should have been removed");
        assert!(output_file.exists(), "Output file should have been created");
        assert_eq!(fs::read(&output_file)?, content, "Content should match");

        Ok(())
    }

    #[test]
    fn test_mv_dosecheck_pdfs_filters_and_mixed() -> anyhow::Result<()> {
        let input_dir = tempdir()?;
        let output_dir = tempdir()?;

        // Matching files
        let match1 = input_dir.path().join("DoseCHECK_001.pdf");
        let match2 = input_dir.path().join("DoseCHECK_report_abc.pdf");
        fs::write(&match1, b"pdf 1")?;
        fs::write(&match2, b"pdf 2")?;

        // Non-matching files: wrong prefix, wrong extension, wrong casing
        let wrong_ext = input_dir.path().join("DoseCHECK_data.txt");
        let wrong_prefix = input_dir.path().join("Report_DoseCHECK.pdf");
        let wrong_case = input_dir.path().join("dosecheck_002.pdf");
        fs::write(&wrong_ext, b"txt content")?;
        fs::write(&wrong_prefix, b"wrong prefix content")?;
        fs::write(&wrong_case, b"lowercase content")?;

        // Directory that matches naming pattern
        let sub_dir = input_dir.path().join("DoseCHECK_subfolder.pdf");
        fs::create_dir(&sub_dir)?;

        mv_dosecheck_pdfs(input_dir.path(), output_dir.path())?;

        // Check moved files
        assert!(!match1.exists());
        assert!(!match2.exists());
        assert!(output_dir.path().join("DoseCHECK_001.pdf").exists());
        assert!(output_dir.path().join("DoseCHECK_report_abc.pdf").exists());
        assert_eq!(
            fs::read(output_dir.path().join("DoseCHECK_001.pdf"))?,
            b"pdf 1"
        );
        assert_eq!(
            fs::read(output_dir.path().join("DoseCHECK_report_abc.pdf"))?,
            b"pdf 2"
        );

        // Check unmoved files and directories
        assert!(wrong_ext.exists());
        assert!(wrong_prefix.exists());
        assert!(wrong_case.exists());
        assert!(sub_dir.exists());

        assert!(!output_dir.path().join("DoseCHECK_data.txt").exists());
        assert!(!output_dir.path().join("Report_DoseCHECK.pdf").exists());
        assert!(!output_dir.path().join("dosecheck_002.pdf").exists());
        assert!(!output_dir.path().join("DoseCHECK_subfolder.pdf").exists());

        Ok(())
    }

    #[test]
    fn test_mv_dosecheck_pdfs_empty_input_directory() -> anyhow::Result<()> {
        let input_dir = tempdir()?;
        let output_dir = tempdir()?;

        let result = mv_dosecheck_pdfs(input_dir.path(), output_dir.path());
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_mv_dosecheck_pdfs_nonexistent_input_fails() {
        let input_dir = tempdir().unwrap();
        let output_dir = tempdir().unwrap();
        let nonexistent_input = input_dir.path().join("does_not_exist");

        let result = mv_dosecheck_pdfs(&nonexistent_input, output_dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_mv_dosecheck_pdfs_nonexistent_output_fails_when_copying() -> anyhow::Result<()> {
        let input_dir = tempdir()?;
        let output_dir = tempdir()?;
        let nonexistent_output = output_dir.path().join("nonexistent_subfolder");

        let matching_file = input_dir.path().join("DoseCHECK_sample.pdf");
        fs::write(&matching_file, b"content")?;

        let result = mv_dosecheck_pdfs(input_dir.path(), &nonexistent_output);
        assert!(result.is_err());

        Ok(())
    }
}
