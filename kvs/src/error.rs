//! Module for error handling

use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// The error type for this crate.
#[derive(Debug)]
pub enum ErrorKind {
    /// An IO error
    Io(std::io::Error),
    /// BSON serialization error
    Serialization(bson::ser::Error),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Io(ref err) => err.fmt(f),
            ErrorKind::Serialization(ref err) => err.fmt(f),
        }
    }
}

impl Error for ErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErrorKind::Io(ref err) => Some(err),
            ErrorKind::Serialization(ref err) => Some(err),
        }
    }
}

impl From<std::io::Error> for ErrorKind {
    fn from(err: std::io::Error) -> Self {
        ErrorKind::Io(err)
    }
}

impl From<bson::ser::Error> for ErrorKind {
    fn from(err: bson::ser::Error) -> Self {
        ErrorKind::Serialization(err)
    }
}

/// A specialized `Result` type for this crate.
pub type Result<T> = std::result::Result<T, ErrorKind>;
