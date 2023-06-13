use crate::engines::{Engine, KvStore, KvsEngine};
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
    engine: KvStore,
    addr: SocketAddr,
}

impl KvsServer {
    /// Create a `KvsServer` with a given storage engine and socket address.
    pub fn new(engine: Engine, addr: SocketAddr) -> Result<Self> {
        let kv_store = KvStore::open(".")?;

        Ok(KvsServer {
            engine: kv_store,
            addr,
        })
    }

    /// Listen to the given socket address.
    pub fn listen(&mut self) -> Result<()> {
        let listener = TcpListener::bind(self.addr)?;

        for stream in listener.incoming() {
            let stream = stream?;
            let reader = Deserializer::from_reader(&stream).into_iter::<Request>();

            for request in reader {
                let request = request?;
                let writer = BufWriter::new(&stream);

                self.handle_request(writer, request)?;
            }
        }
        Ok(())
    }

    fn handle_request(&mut self, writer: BufWriter<&TcpStream>, request: Request) -> Result<()> {
        debug!("Received: {:?}", request);

        match request {
            Request::Ping => {
                self.send_response(writer, Response::Pong)?;
            }
            Request::Get { key } => {
                match self.engine.get(key) {
                    Ok(value) => {
                        self.send_response(writer, Response::Value(value))?;
                    }
                    Err(e) => {
                        self.send_response(writer, Response::Error(e.to_string()))?;
                    }
                };
            }
            Request::Set { key, value } => match self.engine.set(key, value) {
                Ok(_) => self.send_response(writer, Response::Success)?,
                Err(e) => {
                    self.send_response(writer, Response::Error(e.to_string()))?;
                }
            },
            Request::Remove { key } => {
                match self.engine.remove(key) {
                    Ok(_) => {
                        self.send_response(writer, Response::Success)?;
                    }
                    Err(e) => {
                        self.send_response(writer, Response::Error(e.to_string()))?;
                    }
                };
            }
        };
        Ok(())
    }

    fn send_response(
        &mut self,
        mut writer: BufWriter<&TcpStream>,
        response: Response,
    ) -> Result<()> {
        debug!("Sending to client: {:?}", response);

        serde_json::to_writer(&mut writer, &response)?;
        writer.flush()?;
        Ok(())
    }
}
