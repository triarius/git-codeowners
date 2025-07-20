mod owners;
mod parser;

use color_eyre::Result;
use ignore::gitignore::GitignoreBuilder;
use std::{collections::HashMap, io::stdin};

fn main() -> Result<()> {
    let codeowner_path = ".github/CODEOWNERS";
    let cwd = std::env::current_dir()?;
    let codeowner = std::fs::read_to_string(codeowner_path)?;

    let owners = parser::parse(&codeowner);

    let owners_by_glob = owners
        .clone()
        .into_iter()
        .map(|owned_path| (owned_path.path.clone(), owned_path))
        .collect::<HashMap<String, parser::OwnedPath>>();

    let mut builder = GitignoreBuilder::new(cwd);
    let builder = owners
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

    owners::find(builder.build()?, owners_by_glob, paths).for_each(|(path, owners)| {
        if !owners.is_empty() {
            println!("{path}: {}", owners.join(", "));
        } else {
            println!("{path}: No owners found");
        }
    });

    Ok(())
}
