use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Application definition to defer to set of commands under [Commands]
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

/// Commands to run inside this program
#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate messages
    Generate {
        /// filename to write content to
        filename: String,

        /// Number of records
        #[arg(long, default_value_t = 1)]
        count: u32,

    },
    /// Show schema for object
    Schema,
    /// Show schema for Vec of object
    SchemaList,
    /// Validate file against schema
    Validate {
        /// filename to read content from
        filename: String,
    },
    /// Receive json file via http
    Receive {
        /// Sets a custom config file
        #[arg(short, long, value_name = "FILE")]
        config: PathBuf,
    },
}

fn main() {

    let args = Args::parse();
    match args.command {
        Commands::Generate { filename, count } => todo!(),
        Commands::Schema => todo!(),
        Commands::SchemaList => todo!(),
        Commands::Validate { filename } => todo!(),
        Commands::Receive { config } => todo!(),
    };
}
