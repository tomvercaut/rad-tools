use clap::Parser;
use pathdiff::diff_paths;
use rad_tools_cp_dcm::{dcm_cp_file, DcmcpError};
use std::path::Path;
use tracing::{debug, error, trace, Level};
use walkdir::WalkDir;

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "
A command line interface (CLI) application to copy DICOM files by patient ID.
"
)]
struct Cli {
    /// File(s) or director(y/ies) from where DICOM files are copied (recursively).
    #[arg(required = true, value_name = "SOURCE")]
    input: Vec<String>,
    /// Directory to where DICOM files are copied.
    #[arg(required = true, value_name = "DST")]
    output: String,
    /// Patient ID (unique patient identifier)
    #[arg(short, long, value_name = "PATIENT_ID")]
    patient_id: String,
    /// Enable logging at INFO level.
    #[arg(short, long, default_value_t = false)]
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
    let level = if cli.trace {
        Level::TRACE
    } else if cli.debug {
        Level::DEBUG
    } else if cli.verbose {
        Level::INFO
    } else {
        Level::WARN
    };
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_target(true)
        .with_max_level(level)
        .init();

    trace!("Commandline arguments: {:#?}", &cli);

    for input in &cli.input {
        let input_path = Path::new(input);
        if !input_path.exists() {
            panic!("Input path [{:#?}] doesn't exist", input);
        }
        let output_dir_path = Path::new(&cli.output);
        if !output_dir_path.exists() {
            panic!("Output path [{:#?}] doesn't exist", &cli.output);
        }
        if !output_dir_path.is_dir() {
            panic!("Output path [{:#?}] is not a directory", &cli.output);
        }
        if input_path == output_dir_path {
            debug!("Input path is the same as the output path.");
        }

        let dcm_cp = |input_path: &Path, output_dir_path: &Path, patient_id: &str| match dcm_cp_file(
            input_path,
            output_dir_path,
            patient_id,
        ) {
            Ok(_) => {}
            Err(e) => match *e {
                DcmcpError::PatientIdNotFound(_) => {}
                _ => {
                    error!("{:#?}", e);
                }
            },
        };

        if input_path.is_file() {
            trace!("Input path [{:#?}] is a file", input_path);
            dcm_cp(input_path, output_dir_path, &cli.patient_id);
        } else if input_path.is_dir() {
            let entries = WalkDir::new(input_path);
            for entry in entries {
                if entry.is_err() {
                    panic!(
                        "Error while walking through {:#?}: {:#?}",
                        input_path,
                        entry.err()
                    );
                }
                let entry = entry.unwrap();
                let entry_path = entry.path();
                if !entry_path.is_file() {
                    continue;
                }
                let rel_path = diff_paths(entry_path, output_dir_path).unwrap();
                let output_path = output_dir_path.join(rel_path);
                dcm_cp(entry_path, &output_path, &cli.patient_id);
            }
        }
    }
}
