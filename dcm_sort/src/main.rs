use std::path::{Path, PathBuf};

use clap::Parser;
use dicom_core::Tag;
use dicom_dictionary_std::tags::{
    MODALITY, PATIENT_ID, PIXEL_DATA, SERIES_DESCRIPTION, SERIES_INSTANCE_UID, SERIES_NUMBER,
    STUDY_DESCRIPTION, STUDY_INSTANCE_UID,
};
use dicom_object::{FileDicomObject, InMemDicomObject, OpenFileOptions};
use tracing::{debug, error, info, trace, Level};
use walkdir::WalkDir;

/// A command line interface (CLI) application to sort DICOM files into a set of subdirectories.
///
/// A command line interface (CLI) application to sort DICOM files into a set of subdirectories.
/// The DICOM data will be sorted by:
/// - Patient ID
/// - Study ID
/// - Series ID
/// - Series Number
/// - Modality
#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "
A command line interface (CLI) application to sort DICOM files into a set of subdirectories.

The DICOM data will be sorted by:
- Patient ID
- Study ID
- Series ID
- Series Number
- Modality
"
)]
struct Cli {
    /// Directory from where DICOM files are read.
    #[arg(short, long, value_name = "DIR")]
    input: String,
    /// Directory to where DICOM files are copied to.
    #[arg(short, long, value_name = "DIR")]
    output: String,
    /// Enable logging at DEBUG level.
    #[arg(long, default_value_t = false)]
    debug: bool,
    /// Enable logging at TRACE level.
    #[arg(long, default_value_t = false)]
    trace: bool,
}

#[derive(Clone, Debug)]
struct Data {
    patient_id: String,
    study_uid: String,
    study_descr: String,
    series_uid: String,
    series_descr: String,
    series_nr: String,
    modality: String,
}

impl Data {
    pub fn to_path_buf<P>(&self, p: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        let p = p.as_ref();
        p.join(&self.patient_id)
            .join(
                if self.study_uid.is_empty() && self.study_descr.is_empty() {
                    "STUDY_UID_UNKNOWN"
                } else if !self.study_descr.is_empty() {
                    &self.study_descr
                } else {
                    &self.study_uid
                },
            )
            .join(
                if self.series_uid.is_empty() && self.series_descr.is_empty() {
                    "SERIES_UID_UNKNOWN"
                } else if !self.series_descr.is_empty() {
                    &self.series_descr
                } else {
                    &self.series_uid
                },
            )
            .join(if self.series_nr.is_empty() {
                "SERIES_NUMBER_UNKNOWN"
            } else {
                &self.series_nr
            })
            .join(if self.modality.is_empty() {
                "MODALITY_UNKNOWN"
            } else {
                &self.modality
            })
    }
}

fn main() {
    let cli = Cli::parse();
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

    let entries = WalkDir::new(&cli.input);

    debug!("Input directory: {:#?}", &cli.input);
    for entry in entries {
        let entry = entry.expect("Unable to get file path entry.");
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let r = OpenFileOptions::new()
            .read_until(PIXEL_DATA)
            // .read_until(SERIES_DESCRIPTION)
            .open_file(path);
        if r.as_ref().is_err() && level >= Level::INFO {
            error!("Error reading DICOM data from: {:#?}", r.as_ref().err());
            continue;
        }
        let obj = r.unwrap();

        let patient_id = get_str(&obj, PATIENT_ID, path);
        let study_uid = get_str(&obj, STUDY_INSTANCE_UID, path);
        let study_descr = get_str(&obj, STUDY_DESCRIPTION, path);
        let series_uid = get_str(&obj, SERIES_INSTANCE_UID, path);
        let series_descr = get_str(&obj, SERIES_DESCRIPTION, path);
        let series_nr = get_str(&obj, SERIES_NUMBER, path);
        let modality = get_str(&obj, MODALITY, path);
        let data = Data {
            patient_id,
            study_uid,
            study_descr,
            series_uid,
            series_descr,
            series_nr,
            modality,
        };
        trace!("Data read from: {:#?}\n{:#?}", path, &data);
        let odir = data.to_path_buf(&cli.output);
        debug!("Output directory: {:#?}", &odir);
        std::fs::create_dir_all(&odir)
            .unwrap_or_else(|e| panic!("Error occurred while creating: {:#?}\n{:#?}", &odir, e));
        let ofile = odir.join(path.file_name().unwrap());
        debug!("Output file: {:#?}", &ofile);
        info!("Copying {:#?} to {:#?}", path, &ofile);
        std::fs::copy(path, &ofile).unwrap_or_else(|e| {
            panic!(
                "Error occurred while copying: {:#?} to {:#?}\n{:#?}",
                &path, &ofile, e
            )
        });
    }
}

fn get_str(obj: &FileDicomObject<InMemDicomObject>, tag: Tag, path: &Path) -> String {
    obj.element(tag)
        .unwrap_or_else(|e| {
            panic!(
                "{:?} not found in DICOM file: {:#?}\n{:#?}",
                tag.to_string(),
                path,
                e
            )
        })
        .to_str()
        .unwrap_or_else(|e| {
            panic!(
                "{:?} cannot be converted from DICOM file: {:#?}\n{:#?}",
                tag.to_string(),
                path,
                e
            )
        })
        .to_string()
        .trim()
        .to_string()
}
