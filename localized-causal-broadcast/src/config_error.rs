use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::result;

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    EmptyFile,
    NoMessageCount,
    NoReceiverID,
    WrongArgsNumber,
    ParseInt(std::num::ParseIntError),
    UndefinedNodeID(u32),
    InvalidHostsFile,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ConfigError::Io(ref err) => err.fmt(f),
            ConfigError::EmptyFile => write!(f, "Config file is empty"),
            ConfigError::NoMessageCount => write!(f, "No message count in config file"),
            ConfigError::NoReceiverID => write!(f, "No receiver ID in config file"),
            ConfigError::ParseInt(ref err) => err.fmt(f),
            ConfigError::WrongArgsNumber => write!(f, "Wrong number of arguments"),
            ConfigError::UndefinedNodeID(id) => write!(f, "Undefined node ID: {}", id),
            ConfigError::InvalidHostsFile => write!(f, "Invalid hosts file"),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ConfigError::Io(ref err) => Some(err),
            ConfigError::ParseInt(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        ConfigError::Io(err)
    }
}

impl From<std::num::ParseIntError> for ConfigError {
    fn from(err: std::num::ParseIntError) -> ConfigError {
        ConfigError::ParseInt(err)
    }
}

pub type Result<T> = result::Result<T, ConfigError>;
