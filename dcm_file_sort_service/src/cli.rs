use clap::Args;
use clap::Parser;

#[allow(rustdoc::invalid_html_tags)]
/// An application to sort DICOM data from an input directory into an output directory.
///
/// An application to sort DICOM data from an input directory into an output directory.
/// The data is sorted based on the date of birth and the patient ID (format: <output dir>/<MMDD>/<patient ID>).
/// The name of the DICOM file is based on the modality and the (unique) SOP instance UID.
#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "
An application to sort DICOM data from an input directory into an output directory.

An application to sort DICOM data from an input directory into an output directory.
The data is sorted based on the date of birth and the patient ID (format: <output dir>/<MMDD>/<patient ID>).
The name of the DICOM file is based on the modality and the (unique) SOP instance UID."
)]
pub struct Cli {
    #[command(flatten)]
    pub manual_args: Option<ManualArgs>,

    #[arg(short, long, group = "conf", conflicts_with = "ManualArgs")]
    pub config: Option<String>,
}

#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
pub struct ManualArgs {
    /// Directory where the DICOM data are read from.
    #[arg(short, long)]
    pub input_dir: String,
    /// Directory where the DICOM data is written to.
    #[arg(short, long)]
    pub output_dir: String,
    /// Directory where files that couldn't be processed are moved.
    #[arg(short, long)]
    pub unknown_dir: String,
    #[arg(long)]
    /// Path generator for DICOM data (accepted values: [DicomPathGeneratorType])
    pub dicom_path_gen: String,
    #[arg(long)]
    /// Enable logging at INFO level.
    #[arg(long, default_value_t = false)]
    pub verbose: bool,
    /// Enable logging at DEBUG level.
    #[arg(long, default_value_t = false)]
    pub debug: bool,
    /// Enable logging at TRACE level.
    #[arg(long, default_value_t = false)]
    pub trace: bool,
}

#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = false)]
pub struct ConfigArgs {
    #[arg(short, long)]
    pub config: String,
}
