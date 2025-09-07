use crate::DicomListener::Dcmtk;
use crate::Error;
use crate::config::DicomStreamEndpoint;
use rad_tools_common::system::which;
use rad_tools_common::{Start, Stop};
use std::process::Child;
use tracing::error;

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
    pub endpoint: &'a DicomStreamEndpoint,
}

impl Start<crate::Result<()>> for DcmtkListener {
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
}

impl Stop<crate::Result<()>> for DcmtkListener {
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

#[derive(Debug)]
pub enum DicomListener {
    Dcmtk(DcmtkListener),
}

impl From<&crate::config::DicomListener> for DicomListener {
    fn from(value: &crate::config::DicomListener) -> Self {
        Dcmtk(DcmtkListener {
            name: value.name.clone(),
            port: value.port,
            ae: value.ae.clone(),
            output: value.output.clone(),
            proc: None,
        })
    }
}

impl Start<crate::Result<()>> for DicomListener {
    fn start(&mut self) -> crate::Result<()> {
        match self {
            DicomListener::Dcmtk(listener) => listener.start(),
        }
    }
}

impl Stop<crate::Result<()>> for DicomListener {
    fn stop(&mut self) -> crate::Result<()> {
        match self {
            DicomListener::Dcmtk(listener) => listener.stop(),
        }
    }
}

impl DicomListener {
    pub fn name(&self) -> &str {
        match self {
            DicomListener::Dcmtk(listener) => listener.name.as_str(),
        }
    }
}
