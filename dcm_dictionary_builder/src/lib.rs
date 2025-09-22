mod error;
pub mod tag;
mod utils;

pub use utils::{create_rs_file, create_tag_dictionary};

pub use error::{Error, Result};
