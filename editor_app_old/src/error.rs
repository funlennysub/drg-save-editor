use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EditorCore(#[from] editor_core::error::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
}
