#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Polygon requires at least 3 points")]
    PolygonRequiresAtLeast3Points,
    #[error("Unable to convert &str into a rotation direction")]
    StrToRotationDirectionError,
}

pub type Result<T> = std::result::Result<T, Error>;