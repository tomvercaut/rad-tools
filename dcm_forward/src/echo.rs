use crate::Error::{
    DcmtkEchoAssociationAborted, DcmtkEchoCannotInitNetwork, DcmtkEchoCommandlineSyntaxError,
    DcmtkEchoOtherError,
};
use crate::listener::DcmtkDicomEcho;
use rad_tools_common::system::which;
use tracing::error;

pub trait DicomEcho {
    fn echo(&self) -> crate::Result<()>;
}
impl DicomEcho for DcmtkDicomEcho<'_> {
    fn echo(&self) -> crate::Result<()> {
        which("echoscu")?;
        let mut cmd = std::process::Command::new("echoscu");
        cmd.arg("-q");
        cmd.arg(self.endpoint.addr.clone());
        cmd.arg(self.endpoint.port.to_string());
        match cmd.status() {
            Ok(status) => {
                let code = status.code().unwrap_or(-1);
                match code {
                    0 => Ok(()),
                    1 => Err(DcmtkEchoCommandlineSyntaxError),
                    60 => Err(DcmtkEchoCannotInitNetwork),
                    70 => Err(DcmtkEchoAssociationAborted),
                    _ => Err(DcmtkEchoOtherError),
                }
            }
            Err(e) => {
                error!("Unable to execute echoscu: {e:#?}");
                Err(DcmtkEchoOtherError)
            }
        }
    }
}
