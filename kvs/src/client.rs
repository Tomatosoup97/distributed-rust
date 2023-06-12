use crate::error::Result;
use crate::requests::{Request, Response};
use serde::Deserialize;
use serde_json::de::{Deserializer, IoRead};
use slog_scope::debug;
use std::io::{BufReader, BufWriter, Write};
use std::net::SocketAddr;
use std::net::TcpStream;

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
        // Send ping to server
        let request = Request::Ping;
        debug!("Sending: {:?}", request);
        serde_json::to_writer(&mut self.writer, &request)?;
        self.writer.flush()?;

        let response = Response::deserialize(&mut self.reader)?;
        debug!("Received from server: {:?}", response);
        Ok(())
    }
}
