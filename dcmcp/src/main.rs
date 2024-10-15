use clap::Parser;
use log::error;
use rad_tools_cp_dcm::{dcm_cp_files, DcmcpError};
use tracing::{trace, Level};

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "
A command line interface (CLI) application to copy DICOM files by patient ID.
"
)]
pub struct Cli {
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
    pub verbose: bool,
    /// Enable logging at DEBUG level.
    #[arg(long, default_value_t = false)]
    pub debug: bool,
    /// Enable logging at TRACE level.
    #[arg(long, default_value_t = false)]
    pub trace: bool,
}

fn main() -> anyhow::Result<()> {
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

    let mut has_errors = 0;
    match dcm_cp_files(&cli.input, &cli.output, &cli.patient_id) {
        Ok(_) => {}
        Err(v) => {
            for be in v {
                match be.as_ref() {
                    // DcmcpError::PathDoesNotExist(_) => {}
                    // DcmcpError::PathNotDir(_) => {}
                    // DcmcpError::InputOutputDirectoryEqual(_) => {}
                    // DcmcpError::InputNotFile(_) => {}
                    // DcmcpError::UnableToCreateDestinationDirectory(_) => {}
                    // DcmcpError::DestinationNotDirectory(_) => {}
                    // DcmcpError::PatientIdCastError(_) => {}
                    // DcmcpError::PatientIdNoMatch(_) => {}
                    // DcmcpError::ReadData(_, _) => {}
                    // DcmcpError::IO(_) => {}
                    // DcmcpError::DestinationNotWritable(_) => {}
                    DcmcpError::PatientIdNotFound(_) => {}
                    e => {
                        has_errors += 1;
                        error!("Error: {}", e);
                    }
                }
            }
        }
    }
    if has_errors > 0 {
        if has_errors == 1 {
            Err(anyhow::anyhow!(
                "An error has been detected while copying the DICOM data."
            ))
        } else {
            Err(anyhow::anyhow!(
                "Errors have been detected while copying the DICOM data."
            ))
        }
    } else {
        Ok(())
    }
}
