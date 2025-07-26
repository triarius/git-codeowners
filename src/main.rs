mod owners;
mod parser;

use clap::Parser;
use color_eyre::Result;
use std::io::stdin;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the CODEOWNERS file
    #[arg(short, long, default_value = ".github/CODEOWNERS")]
    path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let codeowner = std::fs::read_to_string(args.path)?;
    let codeowners = parser::parse(&codeowner);
    let paths = stdin().lines().filter_map(|line| {
        let line = line.ok()?;
        if line.is_empty() {
            return None;
        }
        Some(line.to_string())
    });

    owners::find_and_print(codeowners, paths)
}
