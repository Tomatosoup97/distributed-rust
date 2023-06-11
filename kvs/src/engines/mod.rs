use crate::error::Result;

mod kvs;
pub use self::kvs::KvStore;

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
