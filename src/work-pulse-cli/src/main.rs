mod activity_service;
mod category_mapper;
mod category_service;
mod csv_import;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Work-Pulse CLI", version = "1.0", author = "Walter Stocker <wrstocke@googlemail.com>", about = "A CLI tool for interacting with work-pulse.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Import a CSV file with activities.
    CsvImport {
        /// The path to the CSV file to import.
        #[arg(short, long)]
        file: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::CsvImport { file } => {
            csv_import::import(&file)?;
        }
    }

    Ok(())
}
