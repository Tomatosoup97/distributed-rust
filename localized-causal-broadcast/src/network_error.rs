use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::result;

#[derive(Debug)]
pub enum NetworkError {
    Io(std::io::Error),
    UndefinedNodeID(u32),
    EncodingError(std::string::FromUtf8Error),
    SerializationError(bincode::ErrorKind),
    ChannelError,
}

impl Display for NetworkError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            NetworkError::Io(ref err) => err.fmt(f),
            NetworkError::UndefinedNodeID(id) => write!(f, "Undefined node ID: {}", id),
            NetworkError::EncodingError(ref err) => err.fmt(f),
            NetworkError::SerializationError(ref err) => err.fmt(f),
            NetworkError::ChannelError => write!(f, "Channel error"),
        }
    }
}

impl Error for NetworkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            NetworkError::Io(ref err) => Some(err),
            NetworkError::EncodingError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for NetworkError {
    fn from(err: std::io::Error) -> NetworkError {
        NetworkError::Io(err)
    }
}

impl From<std::string::FromUtf8Error> for NetworkError {
    fn from(err: std::string::FromUtf8Error) -> NetworkError {
        NetworkError::EncodingError(err)
    }
}

impl From<Box<bincode::ErrorKind>> for NetworkError {
    fn from(err: Box<bincode::ErrorKind>) -> NetworkError {
        NetworkError::SerializationError(*err)
    }
}

impl<T> From<std::sync::mpsc::SendError<T>> for NetworkError {
    // TODO: We should propagate the error instead of ignoring it
    fn from(_: std::sync::mpsc::SendError<T>) -> NetworkError {
        NetworkError::ChannelError
    }
}

pub type Result<T> = result::Result<T, NetworkError>;
