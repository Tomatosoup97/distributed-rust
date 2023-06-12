#![allow(dead_code, unused_variables)]

#[macro_use(slog_o)]
extern crate slog;
#[macro_use]
extern crate slog_scope;
extern crate slog_term;

use clap::{Parser, Subcommand};
use kvs::{KvsClient, Result};
use slog::Drain;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::process::exit;

const DEFAULT_LISTENING_ADDRESS: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4000);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Ping {
        #[arg(long, value_name = "ADDR", default_value_t=DEFAULT_LISTENING_ADDRESS)]
        addr: SocketAddr,
    },
    Get {
        key: String,
        #[arg(long, value_name = "ADDR", default_value_t=DEFAULT_LISTENING_ADDRESS)]
        addr: SocketAddr,
    },
    Set {
        key: String,
        value: String,
        #[arg(long, value_name = "ADDR", default_value_t=DEFAULT_LISTENING_ADDRESS)]
        addr: SocketAddr,
    },
    Rm {
        key: String,
        #[arg(long, value_name = "ADDR", default_value_t=DEFAULT_LISTENING_ADDRESS)]
        addr: SocketAddr,
    },
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    info!("Running kvs-client {}", env!("CARGO_PKG_VERSION"));
    info!("------------------------");
    info!("Config: {:?}", cli);

    match cli.command {
        Command::Ping { addr } => {
            let mut client = KvsClient::connect(addr)?;
            client.ping()?;
        }
        Command::Get { key, addr } => {
            let client = KvsClient::connect(addr)?;
        }
        Command::Set { key, value, addr } => {
            let client = KvsClient::connect(addr)?;
        }
        Command::Rm { key, addr } => {
            let client = KvsClient::connect(addr)?;
        }
    }

    Ok(())
}

fn main() {
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let log = slog::Logger::root(slog_term::FullFormat::new(plain).build().fuse(), slog_o!());

    let _guard = slog_scope::set_global_logger(log);
    slog_scope::scope(&slog_scope::logger().new(slog_o!("scope" => "1")), || {
        if let Err(err) = run() {
            eprintln!("{}", err);
            exit(1);
        }
    });
}
