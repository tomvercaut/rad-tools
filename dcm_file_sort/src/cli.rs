use clap::Args;
use clap::Parser;

#[allow(rustdoc::invalid_html_tags)]
/// An application to sort DICOM data from an input directory into an output directory.
///
/// The way data is sorted depends on the path generator used.
/// Currently, the following path generators are supported:
/// * dicom_default: Organizes DICOM files based on the patient ID and the date of birth.
/// * dicom_uzg: Organizes DICOM files based on the patient ID and the date of birth.
///
/// The name of the DICOM file is based on the modality and the (unique) SOP instance UID.
///
/// Logging can be enabled by setting the environment variable RUST_LOG to:
/// * trace
/// * debug
/// * info
/// * warn
/// * error
#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = r#"
An application to sort DICOM data from an input directory into an output directory.

The way data is sorted depends on the path generator used.
Currently, the following path generators are supported:
* dicom_default: Organizes DICOM files based on the patient ID and the date of birth.
* dicom_uzg: Organizes DICOM files based on the patient ID and the date of birth.

The name of the DICOM file is based on the modality and the (unique) SOP instance UID.

Logging can be enabled by setting the environment variable RUST_LOG to:
* trace
* debug
* info
* warn
* error
"#
)]
pub struct Cli {
    #[arg(short, long)]
    pub config: String,
}

#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = false)]
pub struct ConfigArgs {
    #[arg(short, long)]
    pub config: String,
}
