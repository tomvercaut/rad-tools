use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Route {
    pub dir: PathBuf,
    pub endpoints: Vec<crate::endpoint::Endpoint>,
}

