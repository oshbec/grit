use clap::{Parser, Subcommand};
use grit::commands;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "grit")]
#[command(version = "1.0")]
#[command(about = "A rust implementation of git", author = "Josh Bechard")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create an empty Git repository or reinitialize an existing one
    Init {
        /// Where the repository lives
        directory: Option<PathBuf>,
    },
    /// Commit some code
    Commit {
        /// A helpful message to accompany the commit
        #[arg(short)]
        m: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { directory } => {
            commands::init(directory.as_ref());
        }
        Commands::Commit { m } => {
            let message = m.unwrap_or_default();
            commands::commit(&message).unwrap();
        }
    }
}
