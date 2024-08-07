use clap::Parser;
use dcm_sort::TryFromDicomObject;
use dicom_dictionary_std::tags::PIXEL_DATA;
use dicom_object::OpenFileOptions;
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

        let data = dcm_sort::Data::try_from_dicom_obj(&obj)
            .expect("Unable to create Data from DicomObject");
        trace!("Data read from: {:#?}\n{:#?}", path, &data);
        let odir = dcm_sort::to_path_buf(&data, &cli.output).unwrap();
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
