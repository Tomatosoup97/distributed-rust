#![deny(missing_docs)]
#![allow(dead_code, unused_variables)]

//! A simple key/value store library.

pub use error::{ErrorKind, Result};
pub use kv::KvStore;

mod error;
mod kv;
mod log;
