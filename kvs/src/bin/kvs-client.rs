#![allow(dead_code, unused_variables)]

#[macro_use(slog_o)]
extern crate slog;
#[macro_use]
extern crate slog_scope;
extern crate slog_term;

use clap::{Parser, Subcommand};
use kvs::Result;
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
    /// Address to connect to
    #[arg(long, value_name = "ADDR", default_value_t=DEFAULT_LISTENING_ADDRESS)]
    addr: SocketAddr,
}

#[derive(Subcommand, Debug)]
enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    info!("{:?}", cli);
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
