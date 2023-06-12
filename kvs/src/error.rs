//! Module for error handling

use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// The error type for this crate.
#[derive(Debug)]
pub enum ErrorKind {
    /// An IO error
    Io(std::io::Error),
    /// Data serialization error
    Serialization(serde_json::Error),
    /// Conversion error
    ConversionError(String),
    /// Key not found when removing
    KeyNotFound,
    /// Database engine error
    DatabaseEngine(sled::Error),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Io(ref err) => err.fmt(f),
            ErrorKind::Serialization(ref err) => err.fmt(f),
            ErrorKind::ConversionError(str) => write!(f, "ConversionError: {}", str),
            ErrorKind::KeyNotFound => write!(f, "Key not found"),
            ErrorKind::DatabaseEngine(ref err) => err.fmt(f),
        }
    }
}

impl Error for ErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErrorKind::Io(ref err) => Some(err),
            ErrorKind::Serialization(ref err) => Some(err),
            ErrorKind::DatabaseEngine(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ErrorKind {
    fn from(err: std::io::Error) -> Self {
        ErrorKind::Io(err)
    }
}

impl From<serde_json::Error> for ErrorKind {
    fn from(err: serde_json::Error) -> Self {
        ErrorKind::Serialization(err)
    }
}

impl From<sled::Error> for ErrorKind {
    fn from(err: sled::Error) -> Self {
        ErrorKind::DatabaseEngine(err)
    }
}

impl From<std::string::FromUtf8Error> for ErrorKind {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ErrorKind::ConversionError(err.to_string())
    }
}

/// A specialized `Result` type for this crate.
pub type Result<T> = std::result::Result<T, ErrorKind>;
