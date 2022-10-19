use std::process::exit;

use clap::Parser;
use clap::Subcommand;
mod store;
use kvs::store::bitCask;
use store::kvStore::KvStore;
use store::bitCask::BitCask;
use csv;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands
}   

#[derive(Subcommand)]
enum Commands {
    SET {
            key: String,
            value: String,
    },
    GET {
        key: String,
    },
    RM {
        key: String,
    }
}

fn main() {
    /*
    let args = Args::parse();
    match args.command {
        Commands::SET { key, value } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::GET { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::RM { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
    */
    BitCask::read_csv("/home/chenwanqq/rustlearning/kvs/store/0.hint");
}
