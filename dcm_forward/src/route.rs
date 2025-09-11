use std::path::PathBuf;

#[derive(Clone, Debug, Default)]
pub struct Route {
    pub dir: PathBuf,
    pub endpoints: Vec<crate::endpoint::Endpoint>,
}
