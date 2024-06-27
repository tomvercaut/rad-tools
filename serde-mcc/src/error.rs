#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {source}")]
    Io {
        #[from]
        source: ::std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
