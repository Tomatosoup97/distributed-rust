#![allow(dead_code, unused_variables)]

#[macro_use(slog_o)]
extern crate slog;
#[macro_use]
extern crate slog_scope;
extern crate slog_term;

use clap::Parser;
use kvs::{Engine, ErrorKind, KvStore, KvsEngine, KvsServer, Result, SledKvsEngine};
use slog::Drain;
use std::env::current_dir;
use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::process::exit;

const DEFAULT_LISTENING_ADDRESS: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4000);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Address to listen to
    #[arg(long, value_name = "ADDR", default_value_t=DEFAULT_LISTENING_ADDRESS)]
    addr: SocketAddr,

    /// Engine to use
    #[arg(long, value_name="ENGINE", value_enum, default_value_t=Engine::kvs)]
    engine: Engine,
}

fn set_currently_used_engine(engine: Engine) -> Result<()> {
    let engine_file = current_dir()?.join("engine");
    fs::write(engine_file, format!("{:?}", engine))?;
    Ok(())
}

fn run_on_engine<E: KvsEngine>(engine: E, addr: SocketAddr) -> Result<()> {
    set_currently_used_engine(engine.as_type())?;
    let mut server = KvsServer::new(engine, addr)?;
    server.listen()?;

    Ok(())
}

fn currently_used_engine() -> Result<Option<Engine>> {
    let engine = current_dir()?.join("engine");

    if !engine.exists() {
        return Ok(None);
    }

    match fs::read_to_string(engine)?.as_str() {
        "kvs" => Ok(Some(Engine::kvs)),
        "sled" => Ok(Some(Engine::sled)),
        x => {
            warn!("Invalid engine defined in the file: {}", x);
            Ok(None)
        }
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    info!("Running kvs-server {}", env!("CARGO_PKG_VERSION"));
    info!("------------------------");
    info!("Database engine: {:?}", cli.engine);
    info!("Listening address {}", cli.addr);

    match currently_used_engine()? {
        None => {}
        Some(used_engine) => {
            if used_engine != cli.engine {
                return Err(ErrorKind::WrongEngineUsed);
            }
        }
    }

    match cli.engine {
        Engine::kvs => run_on_engine(KvStore::open(".")?, cli.addr),
        Engine::sled => run_on_engine(SledKvsEngine::new(sled::open(".")?), cli.addr),
    }
}

fn main() {
    let plain = slog_term::PlainSyncDecorator::new(std::io::stderr());
    let log = slog::Logger::root(slog_term::FullFormat::new(plain).build().fuse(), slog_o!());

    let _guard = slog_scope::set_global_logger(log);
    slog_scope::scope(&slog_scope::logger().new(slog_o!("scope" => "1")), || {
        if let Err(err) = run() {
            eprintln!("{}", err);
            exit(1);
        }
    });
}
