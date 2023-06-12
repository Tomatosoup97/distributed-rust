use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Ping,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Pong,
}
