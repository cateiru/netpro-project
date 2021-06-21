use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("FileNotfoundError: {0}")]
    FileNotFoundError(String),
}
