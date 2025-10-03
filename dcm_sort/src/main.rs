use clap::Parser;
use dicom_dictionary_std::tags::PIXEL_DATA;
use rad_tools_dcm_sort::{Data, TryFromDicomObject, to_path_buf, unique_dcm_file};
use tracing::{debug, error, info, trace, warn};
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
///
/// Format of the path being created:
/// <output directory>/<patient ID>/<study>/<series>/<series nr>/<modality>
///
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

Format of the path being created:
<output directory>/<patient ID>/<study>/<series>/<series nr>/<modality>
"
)]
struct Cli {
    /// Directory from where DICOM files are read.
    #[arg(short, long, value_name = "DIR")]
    input: String,
    /// Directory to where DICOM files are copied to.
    #[arg(short, long, value_name = "DIR")]
    output: String,
    /// Enable logging at INFO level.
    #[arg(long, default_value_t = false)]
    verbose: bool,
    /// Enable logging at DEBUG level.
    #[arg(long, default_value_t = false)]
    debug: bool,
    /// Enable logging at TRACE level.
    #[arg(long, default_value_t = false)]
    trace: bool,
}

fn main() {
    let cli = Cli::parse();
    let level = rad_tools_common::get_log_level!(cli);
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_target(true)
        .with_max_level(level)
        .init();

    trace!("Commandline arguments: {:#?}", &cli);

    let entries = WalkDir::new(&cli.input);

    debug!("Input directory: {:#?}", &cli.input);
    for entry in entries {
        let dir_entry = match entry {
            Ok(e) => e,
            Err(e) => {
                error!("Error reading directory entry: {:#?}", e);
                continue;
            }
        };
        let path = dir_entry.path();
        if !path.is_file() {
            continue;
        }

        let dicom_open = rad_tools_common::dicom::open_file_until(path, PIXEL_DATA);
        if dicom_open.as_ref().is_err() {
            warn!("Unable to read DICOM data from {:#?}", &path);
            trace!(
                "Unable to read DICOM data from {:#?}: {:#?}",
                path,
                dicom_open.as_ref().err()
            );
            continue;
        }
        let dicom_obj = dicom_open.unwrap();

        let data = match Data::try_from_dicom_obj(&dicom_obj) {
            Ok(d) => d,
            Err(e) => {
                error!(
                    "Unable to create Data from DICOM object for {:#?}: {:#?}",
                    path, e
                );
                continue;
            }
        };

        trace!("Data read from: {:#?}", path);

        let output_dir = match to_path_buf(&data, &cli.output) {
            Ok(p) => p,
            Err(e) => {
                error!(
                    "Unable to resolve output directory for {:#?}: {:#?}",
                    path, e
                );
                continue;
            }
        };

        debug!("Output directory: {:#?}", &output_dir);

        if let Err(e) = std::fs::create_dir_all(&output_dir) {
            error!(
                "Error occurred while creating output directory {:#?}: {:#?}",
                &output_dir, e
            );
            continue;
        }

        let out_file = match unique_dcm_file(data, output_dir.clone()) {
            Ok(f) => f,
            Err(e) => {
                error!(
                    "Unable to compute unique output file in {:#?} for {:#?}: {:#?}",
                    &output_dir, &path, e
                );
                continue;
            }
        };

        info!("Copying {:#?} to {:#?}", path, &out_file);
        if let Err(e) = std::fs::copy(path, &out_file) {
            error!(
                "Error occurred while copying: {:#?} to {:#?}\n{:#?}",
                &path, &out_file, e
            );
            continue;
        }
    }
}
