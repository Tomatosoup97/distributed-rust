#![deny(missing_docs)]
#![allow(dead_code, unused_variables)]

//! A simple key/value store library.

pub use client::KvsClient;
pub use engines::{Engine, KvStore, KvsEngine, SledKvsEngine};
pub use error::{ErrorKind, Result};
pub use server::KvsServer;

mod client;
mod engines;
mod error;
mod log;
mod requests;
mod server;
