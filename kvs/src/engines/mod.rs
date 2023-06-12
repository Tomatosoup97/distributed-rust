use crate::error::Result;
use clap::ValueEnum;

mod kvs;
mod sled;
pub use self::kvs::KvStore;
pub use self::sled::SledKvsEngine;

/// The engine of the key/value store.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Ord, ValueEnum)]
#[allow(non_camel_case_types)]
pub enum Engine {
    /// Key-Value Store database engine
    kvs,
    /// Sled embedded database engine
    sled,
}

/// Storage interface for key-value store.
pub trait KvsEngine {
    /// Sets the value of a string key to a string.
    /// Returns an error if the value is not written successfully.
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// Gets the string value of a string key. If the key does not exist, return None.
    /// Returns an error if the value is not read successfully.
    fn get(&mut self, key: String) -> Result<Option<String>>;

    /// Removes a given string key.
    /// Returns an error if the key does not exit or value is not read successfully.
    fn remove(&mut self, key: String) -> Result<()>;
}
