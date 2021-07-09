use thiserror::*;

/// Error Enum for possible errors
#[derive(Error, Debug)]
pub enum MagnetError {
    /// Can't open file
    #[error("Could not load file: {0}")]
    LoadError(std::io::Error),

    /// When parsing floats fail
    #[error("Could not parse float: {0}")]
    ParseError(std::num::ParseFloatError),

    // #[error("Could not parse float: {0}")]
    // StackError(),
    /// Generic custom errors, string is passed to it
    #[error("Error: {0}")]
    CustomError(&'static str),
}

impl From<std::io::Error> for MagnetError {
    fn from(e: std::io::Error) -> Self {
        MagnetError::LoadError(e)
    }
}

impl From<std::num::ParseFloatError> for MagnetError {
    fn from(e: std::num::ParseFloatError) -> Self {
        MagnetError::ParseError(e)
    }
}

impl From<&'static str> for MagnetError {
    fn from(e: &'static str) -> Self {
        MagnetError::CustomError(e)
    }
}
