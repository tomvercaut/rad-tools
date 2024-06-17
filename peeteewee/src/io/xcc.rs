use log::error;

use crate::data::xcc::Xcc;
use crate::error::PeeTeeWeeError;

use std::path::Path;

pub fn read<P: AsRef<Path>>(path: P) -> Result<Xcc, PeeTeeWeeError> {
    let content = std::fs::read_to_string(path)?;
    serde_xml_rs::from_str(&content).map_err(|e| {
        error!("{:#?}", e);
        PeeTeeWeeError::SerdeDeserializeError(e.to_string())
    })
}
