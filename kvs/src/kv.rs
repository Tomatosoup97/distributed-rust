use crate::error::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// A key-value store.
#[derive(Debug)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    /// Create a new KvStore.
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Set the value of a string key to a string. Return an error if the value is not
    /// written successfully.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let log_entry = LogEntry::add(key, value);
        let serialized_log: bson::Document = bson::to_bson(&log_entry)?;

        let mut v: Vec<u8> = Vec::new();

        println!("{:?}", log_entry);
        println!("{:?}", serialized_log);
        Ok(())
    }

    /// Get the string value of a string key. If the key does not exist, return None.
    /// Return an error if the value is not read successfully.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
    }

    /// Remove a given key. Return an error if the key does not exist or is not removed
    /// successfully.
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.map.remove(&key);
        Ok(())
    }

    /// Open the KvStore at a given path. Return the KvStore.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        todo!()
    }
}

const TOMBSTONE: &str = "__tombstone__";
const TOMBSTONE_SIZE: u64 = TOMBSTONE.len() as u64;

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    timestamp: DateTime<Utc>,
    key_size: u64,
    value_size: u64,
    key: String,
    value: String,
}

impl LogEntry {
    fn add(key: String, value: String) -> Self {
        assert!(value != TOMBSTONE);

        Self {
            timestamp: Utc::now(),
            key_size: key.len() as u64,
            value_size: value.len() as u64,
            key,
            value,
        }
    }

    fn remove(key: String) -> Self {
        Self {
            timestamp: Utc::now(),
            key_size: key.len() as u64,
            value_size: TOMBSTONE_SIZE,
            key,
            value: TOMBSTONE.to_string(),
        }
    }
}
