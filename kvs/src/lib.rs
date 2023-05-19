#![deny(missing_docs)]
#![allow(dead_code, unused_variables)]
//! A simple key/value store library.
use std::collections::HashMap;

/// A key-value store.
#[derive(Debug)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Create a new KvStore.
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Set the value of a string key to a string.
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Get the value of a given key.
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// Remove a given key from the store.
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}
