use clap::Parser;
use dicom_object::ReadError;
use log::{error, warn};
use rad_tools_dcm_cp::{DcmcpError, dcm_cp_files};
use std::io::ErrorKind;
use tracing::trace;

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
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    rad_tools_core::tracing::default_env_subscriber().init();

    trace!("Commandline arguments: {:#?}", &cli);

    let mut has_errors = 0;
    match dcm_cp_files(&cli.input, &cli.output, &cli.patient_id) {
        Ok(_) => {}
        Err(v) => {
            for be in v {
                match be.as_ref() {
                    DcmcpError::ReadData(_p, e) => match e {
                        ReadError::ReadFile {
                            filename: _,
                            backtrace: _,
                            source,
                        } => match source.kind() {
                            ErrorKind::UnexpectedEof => {}
                            _ => {
                                warn!("{:#?}", e);
                            }
                        },
                        e => {
                            has_errors += 1;
                            error!("ReadError: {:#?}", e);
                        }
                    },
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
