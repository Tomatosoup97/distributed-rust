use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Ping,
    Get { key: String },
    Set { key: String, value: String },
    Remove { key: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Success,
    Error(String),
    Value(Option<String>),
    Pong,
}
