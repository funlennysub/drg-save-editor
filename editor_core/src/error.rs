use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Entry {0} is missing")]
    MissingEntry(String),
    #[error("Failed to cast property to {0}")]
    FailedCastError(String),
}

impl ParsingError {
    pub fn missing_entry(entry: &str) -> Self {
        Self::MissingEntry(entry.to_string())
    }

    pub fn failed_cast(to: &str) -> Self {
        Self::FailedCastError(to.to_string())
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parsing(#[from] ParsingError),
    #[error(transparent)]
    Gvas(#[from] gvas::error::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
}
