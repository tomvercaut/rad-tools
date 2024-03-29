use std::path::Path;
use std::time::SystemTime;

use clap::Parser;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;
use tracing::{debug, trace, warn, Level};
use walkdir::{DirEntry, WalkDir};

use rad_tools_ls_dcm::io::read_dicom_file_partial_by_modalities;
use rad_tools_ls_dcm::model::{DicomFile, Modality, SopClass};

/// A command line interface (CLI) application for reading and listing RTPLAN DICOM files.
/// 
/// Application enables the user to specify the directory from which the DICOM files are read,
/// as well as additional options such as filtering by filename prefixes, limiting the number of displayed results,
/// sorting the files by last modified timestamp, and enabling logging at different levels.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "
A command line interface (CLI) application for reading and listing RTPLAN DICOM files.

Application enables the user to specify the directory from which the DICOM files are read,
as well as additional options such as filtering by filename prefixes, limiting the number of displayed results,
sorting the files by last modified timestamp, and enabling logging at different levels.
")]
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

#[tokio::main]
async fn main() {
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

    let has_prefixes = !cli.prefixes.is_empty();
    let mut files = ls_files(cli.dir.as_ref().unwrap(), |entry| -> bool {
        if !entry.path().is_file() {
            return false;
        }
        return if has_prefixes {
            entry
                .file_name()
                .to_str()
                .map(|s| cli.prefixes.iter().any(|prefix| s.starts_with(prefix)))
                .unwrap_or(false)
        } else {
            true
        };
    });
    trace!("List of filtered files:");
    for (_t, entry) in &files {
        trace!("Path: {:#?}", entry.path());
    }
    if cli.sort {
        trace!("Sorting DICOM files by last modified timestamp (reversed).");
        files.sort_by(|(t1, _), (t2, _)| t1.cmp(t2));
        files.reverse();
    }
    let files2: Vec<_> = files.into_iter().map(|(_, entry)| entry).collect();
    let modalities = vec![Modality::RtPlan];
    let dicom_files = read_dicom_files(files2, modalities, cli.limit).await;
    let view_model_items = build_view_model(&dicom_files);
    let view = build_view(&view_model_items);
    println!("{view}");
}

fn build_view(items: &[ViewModelItem]) -> Table {
    trace!("Building tabluar view.");
    let hdr = ["Patient ID", "Patient Name", "Plan Name", "Plan Label"];

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.apply_modifier(UTF8_ROUND_CORNERS);
    table.set_header(hdr);
    for item in items {
        table.add_row([&item.patient_id, &item.patient_name, &item.plan_name, &item.plan_label]);
    }
    table
}

/// Represents a view model item that contains information about a DICOM file.
#[derive(Clone, Debug, Default)]
struct ViewModelItem {
    patient_id: String,
    patient_name: String,
    sop: SopClass,
    plan_name: String,
    plan_label: String,
    dose_grids: usize,
}

fn build_view_model(dfs: &[DicomFile]) -> Vec<ViewModelItem> {
    let mut view = vec![];
    for df in dfs {
        if let DicomFile::RTPlan(plan) = df {
            view.push(ViewModelItem {
                patient_id: plan.patient_id.clone(),
                patient_name: plan.patient_name.clone(),
                sop: plan.sop.clone(),
                plan_name: plan.plan_name.clone(),
                plan_label: plan.plan_label.clone(),
                dose_grids: 0,
            });
        }
    }
    for df in dfs {
        if let DicomFile::RTDose(dose) = df {
            for rsc in &dose.referenced_rtplan_sequence {
                for v in &mut view {
                    if rsc.ref_class_uid == v.sop.class_uid
                        && rsc.ref_instance_uid == v.sop.instance_uid
                    {
                        v.dose_grids += 1;
                    }
                }
            }
        }
    }
    view
}

/// Reads DICOM files asynchronously based on the provided list of files and modalities.
///
/// # Arguments
///
/// * `files` - A vector of `DirEntry` representing the files to be read.
/// * `modalities` - A vector of `Modality` representing the modalities to filter the DICOM data.
/// * `limit` - An optional usize representing the limit of DICOM files to read.
///
/// # Returns
///
/// A vector of `DicomFile` representing the DICOM files read.
async fn read_dicom_files(
    files: Vec<DirEntry>,
    modalities: Vec<Modality>,
    limit: Option<usize>,
) -> Vec<DicomFile> {
    let mut handles = vec![];
    debug!("Starting tasks to read DICOM files");
    for file in &files {
        let t_file = file.clone();
        let t_modalities = modalities.clone();
        let handle = tokio::spawn(async move {
            read_dicom_file_partial_by_modalities(t_file.path().to_path_buf(), t_modalities)
        });
        handles.push(handle)
    }
    debug!("Waiting for all reading tasks to complete");
    let mut dicom_files = vec![];
    for handle in handles {
        match &limit {
            None => {}
            Some(limit) => {
                if dicom_files.len() >= *limit {
                    handle.abort();
                    continue;
                }
            }
        }

        let result_join = handle.await;
        if result_join.as_ref().is_err() {
            trace!("Unable to read DICOM data: {:#?}", result_join.err());
            continue;
        }
        if let Ok(v) = result_join.unwrap() {
            dicom_files.push(v);
        }
    }
    debug!("All reading tasks completed.");
    dicom_files
}

/// Returns a list of files in the specified directory that pass the given filter function.
/// Each file is represented by a tuple containing its last modified timestamp and `DirEntry` struct.
///
/// # Arguments
///
/// * `path` - A path to the directory.
/// * `filter` - A closure that takes a `DirEntry` as parameter and returns a boolean value indicating whether the file should be included in the result
fn ls_files<P: AsRef<Path>, F>(path: P, filter: F) -> Vec<(SystemTime, DirEntry)>
where
    F: Fn(&DirEntry) -> bool,
{
    let path = path.as_ref();
    debug!("Listing files in: {:#?}", path);
    let mut v = vec![];
    for entry in WalkDir::new(path).into_iter().flatten() {
        if filter(&entry) {
            if let Ok(meta) = entry.metadata() {
                if let Ok(modified) = meta.modified() {
                    v.push((modified, entry));
                } else {
                    warn!("Unable to obtain last modified timestamp from {:#?}", entry);
                }
            } else {
                warn!("Unable to obtain meta data from {:#?}", entry);
            }
        }
    }
    debug!("Listing files in: {:#?} completed", path);
    v
}
