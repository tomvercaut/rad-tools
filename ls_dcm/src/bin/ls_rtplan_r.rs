#![allow(unused_imports)]
use rad_tools_ls_dcm::DicomError;
use rayon::prelude::*;
use std::ffi::OsString;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use clap::Parser;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;
use tracing::{debug, error, trace, warn, Level};
use walkdir::WalkDir;

use rad_tools_ls_dcm::io::{read_dicom_file_partial, read_dicom_file_partial_by_modalities};
use rad_tools_ls_dcm::model::{DicomFile, Modality, SopClass};
use rad_tools_ls_dcm::view;

/// A command line interface (CLI) application for reading and listing RTPLAN DICOM files.
///
/// Application enables the user to specify the directory from which the DICOM files are read,
/// as well as additional options such as filtering by filename prefixes, limiting the number of displayed results,
/// sorting the files by last modified timestamp, and enabling logging at different levels.
#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "
A command line interface (CLI) application for reading and listing RTPLAN DICOM files.

Application enables the user to specify the directory from which the DICOM files are read,
as well as additional options such as filtering by filename prefixes, limiting the number of displayed results,
sorting the files by last modified timestamp, and enabling logging at different levels.
"
)]
struct Cli {
    /// Directory from where DICOM files are read (recursively).
    /// If unspecified, the current directory (".") is analysed.
    #[arg(short, long, value_name = "DIR")]
    dir: Option<String>,
    /// If specified, only filenames starting with a matching prefix, will be read.
    /// Specifying this will increase the performance of the application.
    #[arg(short, long)]
    prefixes: Vec<String>,
    /// Limit the number of displayed results.
    #[arg(short, long)]
    limit: Option<usize>,
    /// Sort the reported data by last modified timestamp of the file.
    #[arg(short, long, default_value_t = false)]
    sort: bool,
    /// Enable logging at DEBUG level.
    #[arg(long, default_value_t = false)]
    debug: bool,
    /// Enable logging at TRACE level.
    #[arg(long, default_value_t = false)]
    trace: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Error transversing the directory / file tree.")]
    WalkDir(#[from] walkdir::Error),
    #[error("Directory / file entry filtered out: {0:#?}")]
    FilteredOut(OsString),
    #[error("Unable to obtain last modified timestamp from {0:#?}")]
    ModifiedTimestamp(OsString),
    #[error("IO error while transvering directory tree.")]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    DicomError(#[from] DicomError),
}

fn main() {
    let mut cli = Cli::parse();
    let level = if cli.trace {
        Level::TRACE
    } else if cli.debug {
        Level::DEBUG
    } else {
        Level::WARN
    };
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_target(true)
        .with_max_level(level)
        .init();

    trace!("Commandline arguments: {:#?}", &cli);

    if cli.dir.as_ref().is_none() {
        cli.dir = Some(".".to_string());
    }

    let dir_path = PathBuf::from(cli.dir.as_ref().unwrap());

    let filter = |entry: &walkdir::DirEntry, prefixes: &[String]| -> bool {
        if !entry.path().is_file() {
            return false;
        }
        return if !prefixes.is_empty() {
            entry
                .file_name()
                .to_str()
                .map(|s| prefixes.iter().any(|prefix| s.starts_with(prefix)))
                .unwrap_or(false)
        } else {
            true
        };
    };
    let modalities = vec![Modality::RtPlan];

    trace!("Starting to iterate the directory: {:#?}", &dir_path);
    let par_iter = WalkDir::new(dir_path)
        .into_iter()
        .par_bridge()
        .map(|r| read_dicom_data(r, filter, &cli.prefixes, &modalities));
    let results: Vec<Result<EntryData, AppError>> = par_iter.collect();

    trace!("Filtering out warnings and errors.");
    let mut dataset = vec![];
    for r in results {
        if r.as_ref().is_err() {
            debug!("{:#?}", r);
            continue;
        }
        dataset.push(r.unwrap());
    }

    if cli.sort {
        trace!("Sorting DICOM data by last modified timestamp (reversed).");
        dataset.sort_by(|a, b| a.modified_time.cmp(&b.modified_time));
        dataset.reverse();
    }

    let has_limit = cli.limit.is_some();
    let limit = cli.limit.unwrap_or_default();
    let mut dicom_files = vec![];
    for data in dataset {
        dicom_files.push(data.dicom_file);
        if has_limit && dicom_files.len() >= limit {
            break;
        }
    }

    let view_model = view::build_model(&dicom_files);
    let view = view::build_view(&view_model);
    println!("{view}");
}

#[derive(Clone, Debug)]
struct EntryData {
    modified_time: SystemTime,
    dicom_file: DicomFile,
}

fn read_dicom_data<F>(
    r: Result<walkdir::DirEntry, walkdir::Error>,
    filter: F,
    prefixes: &[String],
    modalities: &[Modality],
) -> Result<EntryData, AppError>
where
    F: Fn(&walkdir::DirEntry, &[String]) -> bool,
{
    let entry = r?;
    if !filter(&entry, prefixes) {
        return Err(AppError::FilteredOut(dir_entry_path(&entry)));
    }
    let metadata = entry.metadata()?;
    let modified_time = metadata.modified().map_err(|e| {
        let filename = dir_entry_path(&entry);
        warn!(
            "Unable to get last modified time of {:#?} due to an error: {:#?}",
            &filename, e
        );
        AppError::ModifiedTimestamp(filename)
    })?;
    let dicom_file = read_dicom_file_partial_by_modalities(entry.path(), modalities)?;

    Ok(EntryData {
        modified_time,
        dicom_file,
    })
}

fn dir_entry_path(entry: &walkdir::DirEntry) -> OsString {
    entry
        .path()
        .file_name()
        .unwrap_or_else(|| std::ffi::OsStr::new("UNKNOWN FILENAME"))
        .to_os_string()
}
