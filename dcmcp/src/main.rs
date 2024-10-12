use clap::Parser;
use rad_tools_cp_dcm::dcm_cp_files;
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

    dcm_cp_files(&cli.input, &cli.output, &cli.patient_id);
}
