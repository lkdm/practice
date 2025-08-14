use thiserror::Error;

use crate::fs::BinaryFileError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to work with binary file: {0}")]
    BinaryFileError(#[from] BinaryFileError),
}
