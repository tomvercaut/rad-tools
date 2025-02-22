const SERVICE_NAME: &str = "DicomFileSortService";
const SERVICE_DISPLAY_NAME: &str = "Dicom File Sort Service";
const SERVICE_DESCRIPTION: &str = "Service to sort DICOM files by patient ID and date of birth";

#[cfg(windows)]
fn main() {
    println!("Start the service");
}

#[cfg(not(windows))]
fn main() {
    panic!("This service is only intented to run on Windows");
}
