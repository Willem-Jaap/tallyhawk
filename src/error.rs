use std::fmt;

#[derive(Debug)]
pub enum TallyhawkError {
    IoError(std::io::Error),
    SerializationError(serde_json::Error),
    InvalidPath(String),
    InvalidFormat(String),
}

impl fmt::Display for TallyhawkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TallyhawkError::IoError(err) => write!(f, "IO error: {}", err),
            TallyhawkError::SerializationError(err) => write!(f, "Serialization error: {}", err),
            TallyhawkError::InvalidPath(path) => write!(f, "Invalid path: {}", path),
            TallyhawkError::InvalidFormat(format) => write!(f, "Invalid format: {}", format),
        }
    }
}

impl std::error::Error for TallyhawkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TallyhawkError::IoError(err) => Some(err),
            TallyhawkError::SerializationError(err) => Some(err),
            TallyhawkError::InvalidPath(_) => None,
            TallyhawkError::InvalidFormat(_) => None,
        }
    }
}

impl From<std::io::Error> for TallyhawkError {
    fn from(err: std::io::Error) -> Self {
        TallyhawkError::IoError(err)
    }
}

impl From<serde_json::Error> for TallyhawkError {
    fn from(err: serde_json::Error) -> Self {
        TallyhawkError::SerializationError(err)
    }
}

/// Type alias for Results using TallyhawkError
pub type Result<T> = std::result::Result<T, TallyhawkError>;