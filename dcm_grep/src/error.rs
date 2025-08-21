#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid tag format")]
    InvalidTagFormat,
    #[error("Invalid hexadecimal value")]
    InvalidHexValue,
    #[error("Invalid selector range format")]
    InvalidSelectorRangeFormat,
    #[error("Invalid selector format")]
    InvalidSelectorFormat,
    #[error("Invalid search PATTERN format")]
    InvalidSearchPatternFormat,
    #[error("Invalid group format")]
    InvalidGroupFormat,
    #[error("Invalid element format")]
    InvalidElementFormat,
    #[error("Invalid search PATTERN")]
    InvalidSearchPattern,
    #[error("No matching element found according to the search pattern")]
    NoMatchingElementFound,
}
