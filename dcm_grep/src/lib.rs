mod error;
pub mod fmt;
mod grep;
mod pattern;

pub use grep::{GrepMetaResult, GrepResult, element_value_to_string, grep, grep_meta};

pub use error::Error;
