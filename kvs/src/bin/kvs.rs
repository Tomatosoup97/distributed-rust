#![allow(dead_code, unused_variables)]

#[macro_use(slog_o)]
extern crate slog;
extern crate slog_scope;
extern crate slog_term;

use clap::{Parser, Subcommand};
use kvs::Result;
use slog::Drain;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    let mut kv_store = kvs::KvStore::open(".")?;

    match cli.command {
        Command::Get { key } => match kv_store.get(key) {
            Ok(Some(value)) => {
                println!("{}", value);
                exit(0);
            }
            Ok(None) => {
                println!("Key not found");
                exit(0);
            }
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        },
        Command::Set { key, value } => {
            if let Err(e) = kv_store.set(key, value) {
                println!("{}", e);
                exit(1);
            } else {
                exit(0);
            }
        }
        Command::Rm { key } => match kv_store.remove(key) {
            Err(e) => {
                println!("{}", e);
                exit(1);
            }
            Ok(_) => exit(0),
        },
    }
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
