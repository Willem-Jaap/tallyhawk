use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod error;
mod stats;
mod types;
mod utils;

use types::OutputFormat;

#[derive(Parser)]
#[command(name = "tallyhawk")]
#[command(about = "A sharp-eyed CLI tool for gathering project statistics")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Count {
        /// Path to analyze (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        
        /// Include hidden files and directories
        #[arg(short, long)]
        all: bool,
        
        /// Output format
        #[arg(short, long, value_enum, default_value = "table")]
        format: OutputFormat,
        
        /// Respect .gitignore files
        #[arg(long, default_value = "true")]
        gitignore: bool,
        
        /// Include blank lines in count
        #[arg(long)]
        include_blanks: bool,
        
        /// Include comments in count
        #[arg(long)]
        include_comments: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Count {
            path,
            all,
            format,
            gitignore,
            include_blanks,
            include_comments,
        } => {
            let config = commands::count::CountConfig {
                path,
                include_hidden: all,
                output_format: format,
                respect_gitignore: gitignore,
                include_blank_lines: include_blanks,
                include_comments,
            };
            
            commands::count::run(config)?;
        }
    }

    Ok(())
}