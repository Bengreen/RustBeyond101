use std::path::PathBuf;

use clap::{Parser, Subcommand};
use largejson::schema::{Person, schema_string, validate_with_schema};
use largejson::webservice::http_receive_json;
use largejson::{webservice::MyConfig, schema::write_records, error::MyError};
use log::info;
use largejson::{NAME, VERSION};
use largejson::{schema::{schema_person_string}};

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

fn main() -> Result<(), MyError> {
    let log_level = env_logger::Env::default().default_filter_or("info");
    env_logger::Builder::from_env(log_level).init();

    let args = Args::parse();
    match args.command {
        Commands::Generate { filename, count } => {
            println!("Creating filename {filename} and writing {count} records");
            write_records(&filename, count)?;
        },
        Commands::Schema => println!("{}", schema_person_string()?),
        Commands::SchemaList => println!("{}", schema_string::<Vec<Person>>()?),
        Commands::Validate { filename } => validate_with_schema(&filename, 2)?,
        Commands::Receive{ config } => {

            info!("Starting {NAME} for {VERSION}");

            let config: MyConfig = MyConfig::figment(config)
                .extract()
                .expect("Config file loaded");

            info!("Loaded config {:?}", config);

            println!("Loaded config as {:#?}", config);
            http_receive_json(config.webservice)?
        },
    };
    Ok(())
}
