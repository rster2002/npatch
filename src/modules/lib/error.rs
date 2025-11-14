use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LibError {
    FailedToExecuteTool(#[from] std::io::Error),
    FromUTF8Error(#[from] std::string::FromUtf8Error),

    #[error("Failed to run otool: {0}")]
    FailedToRunOTool(#[source] std::io::Error),

    #[error("Failed to parse otool output")]
    FailedToParseOToolOutput,

    #[error("Missing filename")]
    MissingFileName,
}