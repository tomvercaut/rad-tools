use clap::Parser;
use dicom_object::{InMemDicomObject, Tag};
use rad_tools_core::dicom::open_file_until;
use rad_tools_core::fs::read_path_from_stdin;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::path::PathBuf;

/// A command line interface (CLI) application to extract the study and serie instance UID from a DICOM file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Filename to a DICOM file, if not specified, the filename will read from standard input.
    #[arg(value_name = "FILE")]
    input: Option<PathBuf>,
}

/// DICOM study and series instance UIDs.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct InstanceUids {
    /// Study instance UID.
    study: String,
    /// Series instance UID.
    series: String,
}

impl TryFrom<&InMemDicomObject> for InstanceUids {
    type Error = anyhow::Error;

    fn try_from(obj: &InMemDicomObject) -> Result<Self, Self::Error> {
        let study_instance_uid = obj
            .element(Tag(0x0020, 0x000D))?
            .to_str()
            .expect("StudyInstanceUid not found in DICOM dataset.");
        let series_instance_uid = obj
            .element(Tag(0x0020, 0x000E))?
            .to_str()
            .expect("SeriesInstanceUid not found in DICOM dataset.");
        let instance_uids = InstanceUids {
            study: study_instance_uid.into(),
            series: series_instance_uid.into(),
        };
        Ok(instance_uids)
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let filename = cli.input.unwrap_or_else(|| {
        read_path_from_stdin().expect("Failed to read the filename from standard input.")
    });
    let obj = open_file_until(&filename, Tag(0x0020, 0x0011))?;
    let instance_uids = InstanceUids::try_from(&obj.into_inner())?;
    let json = serde_json::to_string_pretty(&instance_uids)?;
    io::stdout()
        .lock()
        .write_all(&format!("{}\n", json).into_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use dicom_core::VR;
    use dicom_object::{InMemDicomObject, Tag};

    use crate::InstanceUids;

    fn get_obj() -> InMemDicomObject {
        let mut obj = InMemDicomObject::new_empty();
        obj.put_str(Tag(0x0020, 0x000D), VR::UI, "1234");
        obj.put_str(Tag(0x0020, 0x000E), VR::UI, "5678");
        obj
    }

    fn get_obj_partial() -> InMemDicomObject {
        let mut obj = InMemDicomObject::new_empty();
        obj.put_str(Tag(0x0020, 0x000D), VR::UI, "1234");
        obj
    }

    fn get_obj_empty() -> InMemDicomObject {
        InMemDicomObject::new_empty()
    }

    #[test]
    fn all_found() {
        let obj = get_obj();
        let instance_uids = InstanceUids::try_from(&obj).unwrap();
        assert_eq!("1234", instance_uids.study.as_str());
        assert_eq!("5678", instance_uids.series.as_str());
    }

    #[test]
    fn partial() {
        let obj = get_obj_partial();
        let r = InstanceUids::try_from(&obj);
        assert!(r.is_err());
    }

    #[test]
    fn empty() {
        let obj = get_obj_empty();
        let r = InstanceUids::try_from(&obj);
        assert!(r.is_err());
    }
}
