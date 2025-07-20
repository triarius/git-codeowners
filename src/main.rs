mod parser;

use color_eyre::Result;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::Match;
use std::collections::HashMap;
use std::io::stdin;

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

    find_owners(builder.build()?, owners_by_glob, paths)
}

fn find_owners<I: Iterator<Item = String>>(
    matcher: Gitignore,
    owners_by_glob: HashMap<String, parser::OwnedPath>,
    paths: I,
) -> Result<()> {
    paths.for_each(|path| match matcher.matched(&path, false) {
        Match::Ignore(glob) => {
            if let Some(owner) = owners_by_glob.get(glob.original()) {
                println!("{path}: {}", owner.owners.join(" "))
            } else {
                println!("{path}: No owner found for this path")
            }
        }
        _ => println!("{path}: No owner"),
    });

    Ok(())
}
