use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod cli;
mod core;
mod detect;
mod lint;
mod output;
mod parsers;

#[derive(Parser)]
#[command(
    name = "vcx",
    version,
    about = "Vibe Coding conteXt manager — Lint, sync, and manage AI coding context files"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan project for AI context files
    Scan {
        /// Project directory (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// Output format: table or json
        #[arg(short, long, default_value = "table")]
        format: String,
    },
    /// Lint context files for problems
    Lint {
        /// Project directory
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// Output format: text or json
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    /// Show context health dashboard
    Status {
        /// Project directory
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Initialize VCX for a project
    Init {
        /// Project directory
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// Tools to create templates for (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tools: Vec<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, format } => {
            let dir = resolve_dir(path);
            cli::scan::execute(&dir, &format)
        }
        Commands::Lint { path, format } => {
            let dir = resolve_dir(path);
            cli::lint::execute(&dir, &format)
        }
        Commands::Status { path } => {
            let dir = resolve_dir(path);
            cli::status::execute(&dir)
        }
        Commands::Init { path, tools } => {
            let dir = resolve_dir(path);
            cli::init::execute(&dir, &tools)
        }
    }
}

fn resolve_dir(path: Option<PathBuf>) -> PathBuf {
    path.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}
