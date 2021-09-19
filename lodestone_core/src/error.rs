use thiserror::*;

/// Error Enum for possible errors
#[derive(Error, Debug)]
pub enum MagnetError {
    /// Can't open file
    #[error("Could not load file: {0}")]
    LoadError(std::io::Error),

    /// When parsing floats fail
    #[error("Could not parse float: {0}")]
    ParseFloatError(std::num::ParseFloatError),

    /// Can't parse TOML struct
    #[error("TOML parse error: {0}")]
    TomlParseError(#[from] toml::de::Error),

    /// SERDE error when unable to parse json struct/file
    #[error("Could not parse file: {0}")]
    ParseError(#[from] serde_json::Error),

    ///Polygon side error
    #[error("There must be at least 3 vertices")]
    PolygonSideError(),

    // #[error("Could not parse float: {0}")]
    // StackError(),
    // /// Generic custom errors, string is passed to it
    // #[error("Error: {0}")]
    // CustomError(&'static str),
    /// Any other error
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<std::io::Error> for MagnetError {
    fn from(e: std::io::Error) -> Self {
        MagnetError::LoadError(e)
    }
}

impl From<std::num::ParseFloatError> for MagnetError {
    fn from(e: std::num::ParseFloatError) -> Self {
        MagnetError::ParseFloatError(e)
    }
}

// impl From<&'static str> for MagnetError {
//     fn from(e: &'static str) -> Self {
//         MagnetError::CustomError(e)
//     }
// }
