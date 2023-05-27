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
    pub key: String,
    pub value: String,
}

impl LogEntry {
    pub fn add(key: String, value: String) -> Self {
        assert!(value != TOMBSTONE);

        Self { key, value }
    }

    pub fn remove(key: String) -> Self {
        Self {
            key,
            value: TOMBSTONE.to_string(),
        }
    }

    pub fn is_tombstone(&self) -> bool {
        self.value == TOMBSTONE
    }
}
