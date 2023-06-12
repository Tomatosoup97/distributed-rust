use crate::engines::Engine;
use crate::error::Result;
use crate::requests::{Request, Response};
use serde_json::Deserializer;
use slog_scope::debug;
use std::io::prelude::*;
use std::io::BufWriter;
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};

/// The server of the key/value store.
pub struct KvsServer {
    engine: Engine,
    addr: SocketAddr,
}

impl KvsServer {
    /// Create a `KvsServer` with a given storage engine and socket address.
    pub fn new(engine: Engine, addr: SocketAddr) -> Result<Self> {
        Ok(KvsServer { engine, addr })
    }

    /// Listen to the given socket address.
    pub fn listen(&mut self) -> Result<()> {
        let listener = TcpListener::bind(self.addr)?;

        for stream in listener.incoming() {
            let stream = stream?;
            let reader = Deserializer::from_reader(&stream).into_iter::<Request>();
            let mut writer = BufWriter::new(&stream);

            for request in reader {
                let request = request?;
                debug!("Received: {:?}", request);

                match request {
                    Request::Ping => {
                        let response = Response::Pong;
                        debug!("Sending to client: {:?}", response);
                        serde_json::to_writer(&mut writer, &response)?;
                        writer.flush()?;
                    }
                    // Request::Get { key } => Ok(()),
                    // Request::Set { key, value } => Ok(()),
                    // Request::Remove { key } => Ok(()),
                };
            }
        }
        Ok(())
    }

    fn handle_request(
        &mut self,
        writer: &mut BufWriter<&TcpStream>,
        request: Request,
    ) -> Result<()> {
        Ok(())
    }
}
