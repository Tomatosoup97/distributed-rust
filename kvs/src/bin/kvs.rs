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
    println!("{:?}", cli);

    let mut kv_store = kvs::KvStore::new();

    match cli.command {
        Command::Get { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Command::Set { key, value } => {
            kv_store.set(key, value)?;
            exit(1);
        }
        Command::Rm { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
