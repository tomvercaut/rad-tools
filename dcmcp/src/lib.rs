use dicom_object::ReadError;
use pathdiff::diff_paths;
use std::path::Path;
use tracing::{debug, error, trace};
use walkdir::WalkDir;

#[derive(thiserror::Error, Debug)]
pub enum DcmcpError {
    #[error("Expected an input file to copy: {0:#?}")]
    InputNotFile(std::path::PathBuf),
    #[error("Unable to create destination directory: {0:#?}")]
    UnableToCreateDestinationDirectory(std::path::PathBuf),
    #[error("Expected a destination to copy the data to: {0:#?}")]
    DestinationNotDirectory(std::path::PathBuf),
    #[error("Patient ID not found in {0:#?}")]
    PatientIdNotFound(std::path::PathBuf),
    #[error("Patient ID cannot be converted from {0:#?}")]
    PatientIdCastError(std::path::PathBuf),
    #[error("Patient ID doesn't match in {0:#?}")]
    PatientIdNoMatch(std::path::PathBuf),
    #[error("Error while reading DICOM data from {0:#?}: {1:#?}")]
    ReadData(std::path::PathBuf, ReadError),
    #[error("IO error detected while copying DICOM data")]
    IO(#[from] std::io::Error),
    #[error("Unable to write to destination: {0:#?}")]
    DestinationNotWritable(std::path::PathBuf),
}

pub type DcmResult<T> = Result<T, Box<DcmcpError>>;


/// Copy a DICOM file(s) and or directories to a destination directory if the patient ID matches.
///
/// # Arguments
///
/// * `input`: input file
/// * `output`: output directory
pub fn dcm_cp_files(inputs: &[String], output: &str, patient_id: &str) {
    for input in inputs {
       dcm_cp_file(input, output, patient_id); 
    }
}

/// Copy a DICOM file to a destination directory if the patient ID matches.
///
/// # Arguments
///
/// * `input`: input file
/// * `output`: output directory
/// * `patient_id`: patient ID to match
///
pub fn dcm_cp_file(input: &str, output: &str, patient_id: &str) {
    let input_path = Path::new(input);
    if !input_path.exists() {
        panic!("Input path [{:#?}] doesn't exist", input);
    }
    let output_dir_path = Path::new(&output);
    if !output_dir_path.exists() {
        panic!("Output path [{:#?}] doesn't exist", &output);
    }
    if !output_dir_path.is_dir() {
        panic!("Output path [{:#?}] is not a directory", &output);
    }
    if input_path == output_dir_path {
        debug!("Input path is the same as the output path.");
    }

    let dcm_cp =
        |input_path: &Path, output_dir_path: &Path, patient_id: &str| match internal::dcm_cp_file(
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
        dcm_cp(input_path, output_dir_path, patient_id);
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
            dcm_cp(entry_path, &output_path, patient_id);
        }
    }
}

mod internal {
    use crate::{DcmResult, DcmcpError};
    use dicom_dictionary_std::tags::{ISSUER_OF_PATIENT_ID, PATIENT_ID};
    use dicom_object::file::ReadPreamble;
    use dicom_object::{InMemDicomObject, OpenFileOptions};
    use log::{error, info, trace};

    /// Read the patient ID from a DICOM file
    ///
    /// # Arguments
    ///
    /// * `p`: DICOM file
    ///
    /// returns: Result<String, DcmcpError>
    /// If the patient ID could be read, it's returned otherwise the error is returned.
    fn read_patient_id_from_file<P>(p: P) -> DcmResult<String>
    where
        P: AsRef<std::path::Path>,
    {
        let p = p.as_ref();
        let open_file_options = OpenFileOptions::new()
            .read_preamble(ReadPreamble::default())
            .read_until(ISSUER_OF_PATIENT_ID);
        let r = open_file_options.open_file(p);
        if let Err(e) = r {
            return Err(Box::new(DcmcpError::ReadData(p.to_path_buf(), e)));
        }
        let obj = r.unwrap();
        get_patient_id_from_obj(&obj, p)
    }

    /// Get the patient ID from a DICOM object.
    ///
    /// # Arguments
    ///
    /// * `obj`: in memory DICOM object
    /// * `p`: path from where the DICOM object was orignally read
    ///
    /// returns: Result<String, DcmcpError>
    /// If the patient ID could be read, it's returned otherwise the error is returned.
    fn get_patient_id_from_obj<P>(obj: &InMemDicomObject, p: P) -> DcmResult<String>
    where
        P: AsRef<std::path::Path>,
    {
        let p = p.as_ref();
        let r = obj.element(PATIENT_ID);
        if r.is_err() {
            error!("{:#?}", r.unwrap_err());
            return Err(Box::new(DcmcpError::PatientIdNotFound(p.to_path_buf())));
        }
        let elem = r.unwrap();
        let r = elem.to_str();
        if r.is_err() {
            error!("{:#?}", r.unwrap_err());
            return Err(Box::new(DcmcpError::PatientIdCastError(p.to_path_buf())));
        }
        let pt_id = r.unwrap();
        Ok(pt_id.trim_end().to_string())
    }

    /// Copy a DICOM file to a destination directory if the patient ID matches.
    ///
    /// # Arguments
    ///
    /// * `src`: source file
    /// * `dst`: destination directory
    /// * `patient_id`: patient ID to match
    ///
    pub(crate) fn dcm_cp_file<P>(src: P, dst: P, patient_id: &str) -> DcmResult<()>
    where
        P: AsRef<std::path::Path>,
    {
        let src = src.as_ref();
        let dst = dst.as_ref();
        trace!("Checking if copying {src:#?} to {dst:#?} is valid");
        if !src.is_file() {
            trace!("Copying {src:#?} to {dst:#?} is not possible: source is not a file");
            return Err(Box::new(DcmcpError::InputNotFile(src.to_path_buf())));
        }

        let pt_id = read_patient_id_from_file(src)?;
        if pt_id != patient_id {
            return Err(Box::new(DcmcpError::PatientIdNoMatch(src.to_path_buf())));
        }

        // Only create the output directory if the file is a DICOM file.
        if !dst.is_dir() {
            trace!("Copying {src:#?} to {dst:#?}: destination directory doesn't exist");
            let r = std::fs::create_dir_all(dst);
            if r.is_err() {
                trace!("Copying {src:#?} to {dst:#?}: destination directory couldn't be created");
            }
        }
        if !dst.is_dir() {
            trace!("Copying {src:#?} to {dst:#?} is not possible: destination is not a directory");
            return Err(Box::new(DcmcpError::DestinationNotDirectory(
                dst.to_path_buf(),
            )));
        }
        if !is_dir_writable(&dst) {
            trace!(
            "Copying {src:#?} to {dst:#?} is not possible: destination directory is not writable"
        );
            return Err(Box::new(DcmcpError::DestinationNotWritable(
                dst.to_path_buf(),
            )));
        }

        let ofile = dst.join(src.file_name().unwrap());
        info!("Copying {:#?} -> {:#?}", src, &ofile);
        let _ = std::fs::copy(src, ofile).map_err(|e| Box::new(DcmcpError::IO(e)));
        Ok(())
    }

    /// Check if a directory is writable.
    ///
    /// # Arguments
    ///
    /// * `p`: path to directory
    ///
    /// returns: bool
    /// Returns true if the directory is writable, otherwise false.
    /// Function doesn't check access control using ACL on Windows.
    fn is_dir_writable<P>(p: &P) -> bool
    where
        P: AsRef<std::path::Path>,
    {
        let p = p.as_ref();
        if !p.is_dir() {
            error!("Path is not a directory {:#?}", p);
            return false;
        }
        match std::fs::metadata(p) {
            Ok(m) => !m.permissions().readonly(),
            Err(e) => {
                error!("Couldn't get metadata for {:#?}: {:#?}", p, e);
                false
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use dicom_core::VR;
        use dicom_dictionary_std::tags::{PATIENT_ID, PATIENT_NAME};
        use dicom_dictionary_std::uids::CT_IMAGE_STORAGE;
        use dicom_object::{FileMetaTableBuilder, InMemDicomObject};
        use log::{trace, warn, LevelFilter};
        use std::path::PathBuf;

        fn init_logger() {
            let _ = env_logger::builder()
                .is_test(true)
                .filter_level(LevelFilter::Trace)
                .try_init();
        }

        #[test]
        fn test_get_patient_id_success() {
            let mut obj = InMemDicomObject::new_empty();
            let ts = "123456";
            obj.put_str(PATIENT_ID, VR::LO, ts);
            let result = super::get_patient_id_from_obj(&obj, PathBuf::from("test.dcm"));
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), ts);
        }

        #[test]
        fn test_get_patient_id_not_found() {
            init_logger();
            let obj = InMemDicomObject::new_empty();
            let result = super::get_patient_id_from_obj(&obj, PathBuf::from("test.dcm"));
            assert!(result.is_err());
            match *result.unwrap_err() {
                super::DcmcpError::PatientIdNotFound(e) => {
                    trace!("PatientIdNotFound was an expected error.");
                }
                _ => panic!("Expected DcmcpError::PatientIdNotFound"),
            }
        }

        #[test]
        fn test_get_patient_id_cast_error() {
            init_logger();
            let mut obj = InMemDicomObject::new_empty();
            let t = "123456";
            let s = "12345";
            obj.put_str(PATIENT_ID, VR::LO, s);
            let result = super::get_patient_id_from_obj(&obj, PathBuf::from("test.dcm"));
            assert!(result.is_ok());
            let x = result.unwrap();
            assert_eq!(x, s);
            assert_ne!(x, t);
        }

        #[test]
        fn test_dcm_cp() {
            init_logger();
            let mut obj = InMemDicomObject::new_empty();

            let s = "12345";
            obj.put_str(PATIENT_ID, VR::LO, s);
            obj.put_str(PATIENT_NAME, VR::PN, "Last^First");

            let temp_dir = std::env::temp_dir();
            assert!(super::is_dir_writable(&temp_dir));

            let filename = "test_dcmcp.dcm";

            // Remove any existing temporary files
            let tmp_input = temp_dir.join(filename);
            if tmp_input.is_file() {
                std::fs::remove_file(&tmp_input).unwrap();
            }
            let tmp_out_dir = temp_dir.join("rad_tools_dcm_cp");
            if tmp_out_dir.is_dir() {
                std::fs::remove_dir_all(&tmp_out_dir).unwrap();
            }
            std::fs::create_dir(&tmp_out_dir).unwrap();
            let tmp_output = tmp_out_dir.join(filename);
            if tmp_output.is_file() {
                std::fs::remove_file(&tmp_output).unwrap();
            }

            // Write a temporary DICOM file
            let file_obj = obj
                .with_meta(
                    FileMetaTableBuilder::new()
                        .transfer_syntax(dicom_transfer_syntax_registry::default().erased().uid())
                        .media_storage_sop_class_uid(CT_IMAGE_STORAGE),
                )
                .unwrap();
            file_obj.write_to_file(tmp_input.as_path()).unwrap();
            assert!(tmp_input.is_file());

            // Copy the temporary DICOM file
            super::dcm_cp_file(&tmp_input, &tmp_out_dir, s).unwrap();

            // Check the copied file exists and compare the byte content to ensure it's the same data
            assert!(tmp_output.is_file());
            let v1 = std::fs::read(&tmp_input).unwrap();
            let v2 = std::fs::read(&tmp_output).unwrap();
            assert_eq!(v1, v2);

            // Remove temporary data
            std::fs::remove_file(&tmp_input).unwrap();
            std::fs::remove_file(&tmp_output).unwrap();
            std::fs::remove_dir_all(&tmp_out_dir).unwrap();
        }
    }
}
