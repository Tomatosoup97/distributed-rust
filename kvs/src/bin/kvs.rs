#![allow(dead_code, unused_variables)]
use clap::{Parser, Subcommand};
use kvs::Result;
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

fn main() -> Result<()> {
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
