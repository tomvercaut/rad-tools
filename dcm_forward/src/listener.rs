use crate::Error;
use crate::config::Endpoint;
use rad_tools_common::system::which;
use std::process::Child;
use tracing::error;

/// DicomListener trait defines the interface for DICOM service providers that can listen
/// for incoming DICOM associations and handle received DICOM files.
///
/// Implementors must provide functionality to:
/// - Start the DICOM listening service
/// - Stop the DICOM listening service gracefully
pub trait DicomListener {
    /// Starts the DICOM listening service.
    ///
    /// # Returns
    /// - `Ok(())` if the service started successfully
    /// - `Err` if the service failed to start
    fn start(&mut self) -> crate::Result<()>;

    /// Stops the DICOM listening service gracefully.
    ///
    /// # Returns
    /// - `Ok(())` if the service stopped successfully
    /// - `Err` if the service failed to stop (gracefully)
    fn stop(&mut self) -> crate::Result<()>;
}

#[derive(Debug)]
pub struct DcmtkListener {
    /// Unique name for the listener
    pub name: String,
    /// Port to listen on
    pub port: u16,
    /// DICOM AE Title
    pub ae: String,
    /// Output directory for DICOM files
    pub output: String,
    // Process handle
    proc: Option<Child>,
}

pub struct DcmtkDicomEcho<'a> {
    pub endpoint: &'a Endpoint,
}

impl DicomListener for DcmtkListener {
    fn start(&mut self) -> crate::Result<()> {
        // Stop any existing listener
        self.stop()?;
        // Check if storescp is installed
        which("storescp")?;

        // Create the command
        let mut cmd = std::process::Command::new("storescp");
        cmd.arg("-aet").arg(self.ae.clone());
        cmd.arg("-od").arg(self.output.clone());
        cmd.arg(self.port.to_string());
        // Start the process
        match cmd.spawn() {
            Ok(child) => {
                self.proc = Some(child);
                Ok(())
            }
            Err(e) => {
                error!("Unable to spawn a process: {e:#?}");
                Err(Error::UnableToStartDicomListener)
            }
        }
    }

    fn stop(&mut self) -> crate::Result<()> {
        if let Some(mut child) = self.proc.take()
            && let Err(e) = child.kill()
        {
            error!("Unable to kill dicom listener: {e:#?}");
            return Err(Error::UnableToKillDicomListener);
        }
        Ok(())
    }
}
