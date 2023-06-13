use crate::error::Result;
use crate::requests::{Request, Response};
use serde::Deserialize;
use serde_json::de::{Deserializer, IoRead};
use slog_scope::debug;
use std::io::{BufReader, BufWriter, Write};
use std::net::SocketAddr;
use std::net::TcpStream;
use std::process::exit;

/// The client of the key/value store.
pub struct KvsClient {
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>,
}

impl KvsClient {
    /// Connect to the server at the given socket address.
    pub fn connect(addr: SocketAddr) -> Result<Self> {
        let tcp_reader = TcpStream::connect(addr)?;
        let tcp_writer = tcp_reader.try_clone()?;

        Ok(KvsClient {
            reader: Deserializer::from_reader(BufReader::new(tcp_reader)),
            writer: BufWriter::new(tcp_writer),
        })
    }

    /// Send ping command to the server
    pub fn ping(&mut self) -> Result<()> {
        self.send_request(Request::Ping)?;
        match self.get_response()? {
            Response::Pong => {}
            Response::Error(msg) => {
                eprintln!("{}", msg);
                exit(1);
            }
            _ => {
                eprintln!("Unexpected response");
                exit(1);
            }
        }
        Ok(())
    }

    /// Get the value of a given string key
    pub fn get(&mut self, key: String) -> Result<()> {
        self.send_request(Request::Get { key })?;

        match self.get_response()? {
            Response::Value(Some(value)) => {
                println!("{}", value);
            }
            Response::Value(None) => {
                println!("Key not found");
            }
            Response::Error(msg) => {
                eprintln!("{}", msg);
                exit(1);
            }
            _ => {
                eprintln!("Unexpected response");
                exit(1);
            }
        }
        Ok(())
    }

    /// Set key to hold the string value
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.send_request(Request::Set { key, value })?;

        match self.get_response()? {
            Response::Success => {}
            Response::Error(msg) => {
                eprintln!("{}", msg);
                exit(1);
            }
            _ => {
                eprintln!("Unexpected response");
                exit(1);
            }
        }
        Ok(())
    }

    /// Remove key from the store
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.send_request(Request::Remove { key })?;

        match self.get_response()? {
            Response::Success => {}
            Response::Error(msg) => {
                eprintln!("{}", msg);
                exit(1);
            }
            _ => {
                eprintln!("Unexpected response");
                exit(1);
            }
        }
        Ok(())
    }

    fn send_request(&mut self, request: Request) -> Result<()> {
        debug!("Sending: {:?}", request);
        serde_json::to_writer(&mut self.writer, &request)?;
        self.writer.flush()?;
        Ok(())
    }

    fn get_response(&mut self) -> Result<Response> {
        let response = Response::deserialize(&mut self.reader)?;
        debug!("Received from server: {:?}", response);
        Ok(response)
    }
}
