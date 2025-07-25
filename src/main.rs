mod owners;
mod parser;

use clap::Parser;
use color_eyre::Result;
use ignore::gitignore::GitignoreBuilder;
use std::{collections::HashMap, io::stdin};

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
    let codeowners_by_glob = codeowners
        .clone()
        .into_iter()
        .map(|owned_path| (owned_path.path.clone(), owned_path.owners))
        .collect::<HashMap<String, Vec<String>>>();

    let mut builder = GitignoreBuilder::new(std::env::current_dir()?);
    let builder = codeowners
        .into_iter()
        .try_fold(&mut builder, |acc, owned_path| {
            acc.add_line(None, &owned_path.path)
        })?;

    let paths = stdin().lines().filter_map(|line| {
        let line = line.ok()?;
        if line.is_empty() {
            return None;
        }
        Some(line.to_string())
    });

    owners::find(builder.build()?, codeowners_by_glob, paths).for_each(|(path, owners)| {
        if !owners.is_empty() {
            println!("{path}: {}", owners.join(", "));
        } else {
            println!("{path}: No owners found");
        }
    });

    Ok(())
}
