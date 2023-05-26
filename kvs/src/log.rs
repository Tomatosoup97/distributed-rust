use serde::{Deserialize, Serialize};
use std::time::SystemTime;

const TOMBSTONE: &str = "__tombstone__";
const TOMBSTONE_SIZE: u64 = TOMBSTONE.len() as u64;

fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    timestamp: u64,
    key_size: u64,
    value_size: u64,
    pub key: String,
    pub value: String,
}

impl LogEntry {
    pub fn add(key: String, value: String) -> Self {
        assert!(value != TOMBSTONE);

        Self {
            timestamp: get_sys_time_in_secs(),
            key_size: key.len() as u64,
            value_size: value.len() as u64,
            key,
            value,
        }
    }

    pub fn remove(key: String) -> Self {
        Self {
            timestamp: get_sys_time_in_secs(),
            key_size: key.len() as u64,
            value_size: TOMBSTONE_SIZE,
            key,
            value: TOMBSTONE.to_string(),
        }
    }

    pub fn is_tombstone(&self) -> bool {
        self.value == TOMBSTONE
    }
}
