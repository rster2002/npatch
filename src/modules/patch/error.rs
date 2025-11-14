use std::path::PathBuf;
use thiserror::Error;
use crate::modules::lib::LibError;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum PatchError {
    IO(#[from] std::io::Error),
    LibError(#[from] LibError),
    FromUTF8Error(#[from] std::string::FromUtf8Error),

    #[error("Could not find match for dependency {0}")]
    MissingMatchFor(PathBuf),

    #[error("Failed to represent path")]
    FailedToRepresentPath,

    #[error("Failed to patch install name tool: exit {0} '{1}'")]
    FailedToPatchInstallNameTool(i32, String),
}