mod error;
mod grep;
mod pattern;

pub use grep::{GrepResult, element_value_to_string, grep};

pub use error::Error;
