use crate::Error;
use crate::endpoint::{DicomStreamEndpoint, DirEndpoint, Endpoint};
use crate::route::Route;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::{Duration, SystemTime};
use tracing::{error, trace};
use walkdir::WalkDir;

#[derive(Copy, Clone, Debug)]
pub struct WorkerOptions {
    pub buffer_size: usize,
    pub duration: Duration,
}

/// Starts a worker that continuously processes DICOM files from a directory and sends them to configured endpoints.
///
/// # Arguments
/// * `route` - Route configuration containing source directory and target endpoints
/// * `status_rx` - Receiver channel for status messages to control worker lifecycle
/// * `options` - Worker configuration options including buffer size and file age threshold
///
/// # Returns
/// * `Ok(())` - If the worker completes successfully
/// * `Err(Error)` - If there was an error during processing
///
/// The worker operates in a continuous loop until signaled to stop via the status channel:
/// 1. Checks for stop signal
/// 2. Collects eligible files from source directory up to buffer size limit
/// 3. Processes files concurrently by sending to all configured endpoints
/// 4. Removes successfully processed files
/// 5. Repeats until stop signal received
pub async fn start_with(
    route: Route,
    status_rx: Receiver<bool>,
    options: WorkerOptions,
) -> crate::Result<()> {
    let dir = route.dir.clone();
    let endpoints = route.endpoints.clone();
    loop {
        // Here we can stop, no data is being at this step.
        if should_stop(&status_rx) {
            break;
        }
        let paths = get_file_paths(options.buffer_size, &dir, options.duration);
        // Here we can stop, only file paths have been collected.
        if should_stop(&status_rx) {
            break;
        }
        // Send it to endpoints concurrently.
        // At this point, the data is being processed and there is no stopping until all data is sent.
        let mut handles = vec![];
        for path in paths {
            let tendpoints = endpoints.clone();
            handles.push(tokio::spawn(send_to_endpoints(path, tendpoints)));
        }
        for handle in handles {
            match handle.await {
                Ok(result) => match result {
                    Ok(path) => {
                        if let Err(e) = std::fs::remove_file(&path) {
                            error!(
                                "Failed to remove a file ({:#?}) after sending to all endpoints: {:#?}",
                                path, e
                            );
                        }
                    }
                    Err(e) => {
                        error!(
                            "Because a file was not sent to all endpoints, it will not be removed: {e:#?}."
                        );
                    }
                },
                Err(e) => {
                    error!("Unable to join a task / thread handle: {e:#?}")
                }
            }
        }
    }
    Ok(())
}

/// Checks if the worker should stop processing based on received status
///
/// # Arguments
/// * `status_rx` - Receiver channel for status messages
///
/// # Returns
/// * `true` if the channel is disconnected, or a true value is received
/// * `false` if the channel is empty or a false value is received
fn should_stop(status_rx: &Receiver<bool>) -> bool {
    status_rx.try_recv().unwrap_or_else(|e| match e {
        TryRecvError::Empty => false,
        TryRecvError::Disconnected => true,
    })
}

/// Sends a DICOM file to multiple endpoints concurrently
///
/// # Arguments
/// * `path` - Path to the DICOM file to send
/// * `endpoints` - List of endpoints to send the file to
///
/// # Returns
/// * `Ok(PathBuf)` - The original path if all data was sent were successful
/// * `Err(Error)` - If any send operation failed
async fn send_to_endpoints(path: PathBuf, endpoints: Vec<Endpoint>) -> crate::Result<PathBuf> {
    let mut vdicom = vec![];
    let mut vdir = vec![];
    for endpoint in endpoints {
        match endpoint {
            Endpoint::Dicom(endpoint) => {
                vdicom.push(storescu(path.clone(), endpoint));
            }
            Endpoint::Dir(endpoint) => {
                vdir.push(copy_to_endpoint(path.clone(), endpoint));
            }
        }
    }
    let mut has_error = false;

    for handle in vdicom {
        if handle.await.is_err() {
            has_error = true;
        }
    }
    for handle in vdir {
        if handle.await.is_err() {
            has_error = true;
        }
    }

    if has_error {
        error!("Failed to send a file ({:#?}) to all endpoints", &path);
        Err(Error::SendToEndpoint)
    } else {
        Ok(path)
    }
}

/// Sends a DICOM file to a DICOM endpoint using DCMTK's storescu tool
///
/// # Arguments
/// * `path` - Path to the DICOM file to send
/// * `endpoint` - DICOM endpoint configuration to send to
///
/// # Returns
/// * `Ok(PathBuf)` - The original path if send was successful
/// * `Err(Error)` - If the send operation failed
async fn storescu(path: PathBuf, endpoint: DicomStreamEndpoint) -> crate::Result<PathBuf> {
    trace!("Sending file ({:#?}) to endpoint {:#?}", &path, &endpoint);
    let mut cmd = std::process::Command::new("storescu");
    cmd.arg("-aec")
        .arg(&endpoint.aec)
        .arg("-aet")
        .arg(&endpoint.aet)
        .arg(&endpoint.addr)
        .arg(endpoint.port.to_string())
        .arg(path.to_str().unwrap());

    match cmd.status() {
        Ok(status) => {
            if !status.success() {
                error!(
                    "Failed to send a DICOM file to endpoint {:#?}: {:#?}",
                    endpoint, status
                );
                Err(Error::DcmtkStorescu)
            } else {
                trace!("Send file ({:#?}) to endpoint {:#?}", &path, &endpoint);
                Ok(path)
            }
        }
        Err(e) => {
            error!(
                "Failed to send a DICOM file to endpoint {:#?}: {:#?}",
                endpoint, e
            );
            Err(Error::DcmtkStorescu)
        }
    }
}

/// Copies a DICOM file to a directory endpoint
///
/// # Arguments
/// * `path` - Path to the DICOM file to copy
/// * `endpoint` - Directory endpoint configuration to copy to
///
/// # Returns
/// * `Ok(PathBuf)` - The original path if copy was successful
/// * `Err(Error)` - If the copy operation failed
async fn copy_to_endpoint(path: PathBuf, endpoint: DirEndpoint) -> crate::Result<PathBuf> {
    trace!("Copying file ({:#?}) to endpoint {:#?}", &path, &endpoint);
    let dest = Path::new(&endpoint.path).join(path.file_name().unwrap());
    if let Err(e) = std::fs::copy(&path, &dest) {
        error!(
            "Failed to copy a file to directory endpoint {}: {:#?}",
            endpoint.name, e
        );
        return Err(Error::from(e));
    }
    trace!("Copied file ({:#?}) to endpoint {:#?}", &path, &endpoint);
    Ok(path)
}

/// Retrieves a list of file paths from a directory that meet eligibility criteria
///
/// # Arguments
/// * `buffer_size` - Maximum number of paths to collect (0 for unlimited)
/// * `dir` - Directory to scan for files
/// * `duration` - Duration threshold for file age eligibility
///
/// # Returns
/// Vector of eligible file paths up to buffer_size limit
fn get_file_paths(buffer_size: usize, dir: &PathBuf, duration: Duration) -> Vec<PathBuf> {
    if buffer_size == 0 {
        let mut paths = vec![];
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if !is_eligible(entry.path(), duration) {
                continue;
            }
            paths.push(entry.path().to_path_buf());
        }
        paths
    } else {
        let mut paths = Vec::with_capacity(buffer_size);
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if !is_eligible(entry.path(), duration) {
                continue;
            }
            if paths.len() >= buffer_size {
                break;
            }
            paths.push(entry.path().to_path_buf());
        }
        paths
    }
}

/// Checks if a file is eligible for processing based on when it was last modified.
///
/// # Arguments
/// * `path` - Path to the file to check
/// * `duration` - Minimum duration between the present and the last-modified timestamp for a file to be eligible.
///
/// # Returns
/// `true` if the file exists, is a regular file, and is older than the duration threshold
fn is_eligible<P>(path: P, duration: Duration) -> bool
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if !path.is_file() {
        return false;
    }

    let metadata = path.metadata();
    if let Err(e) = metadata {
        error!("Failed to get metadata for file ({:#?}): {:#?}", path, e);
        return false;
    }
    let now = SystemTime::now();
    let metadata = metadata.unwrap();
    if let Ok(modified) = metadata.modified() {
        if let Ok(age) = now.duration_since(modified) {
            age > duration
        } else {
            false
        }
    } else {
        false
    }
}
