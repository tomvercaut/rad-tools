#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Polygon requires at least 3 points")]
    PolygonRequiresAtLeast3Points,
}

pub type Result<T> = std::result::Result<T, Error>;