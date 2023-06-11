#![deny(missing_docs)]
#![allow(dead_code, unused_variables)]

//! A simple key/value store library.

pub use engines::{KvStore, KvsEngine};
pub use error::{ErrorKind, Result};

mod engines;
mod error;
mod log;
