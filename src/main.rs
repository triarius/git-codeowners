mod owned;
mod owners;
mod parser;

use clap::{Parser, Subcommand};
use color_eyre::Result;
use std::{fs, io::stdin};

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// A tool to find code owners based on a CODEOWNERS file.
struct Cli {
    /// Path to the CODEOWNERS file.
    #[arg(short, long, default_value = ".github/CODEOWNERS")]
    path: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Find owners for the specified paths.
    /// Reads paths from STDIN if not provided as positional arguments.
    Find { paths: Vec<String> },
    /// Print all files owned by the specified owner.
    OwnedBy {
        owner: String,

        #[arg(short, long, default_value = ".")]
        dir: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let codeowners = parser::parse(&fs::read_to_string(cli.path)?);

    match cli.command {
        Command::Find { paths } => {
            let paths = if paths.is_empty() {
                paths_from_stdin()
            } else {
                paths
            };

            owners::find_and_print(codeowners, paths.into_iter())
        }
        Command::OwnedBy { owner, dir } => owned::by(codeowners, &owner, &dir),
    }
}

fn paths_from_stdin() -> Vec<String> {
    stdin()
        .lines()
        .filter_map(|line| {
            let line = line.ok()?.trim().to_string();
            if line.is_empty() {
                return None;
            }
            Some(line)
        })
        .collect::<Vec<_>>()
}
