mod matcher;
mod parser;

use color_eyre::Result;
use std::io::stdin;

fn main() -> Result<()> {
    let codeowner_path = ".github/CODEOWNERS";
    let cwd = std::env::current_dir()?;
    let codeowner = std::fs::read_to_string(codeowner_path)?;
    let owners = parser::parse(&codeowner)
        .into_iter()
        .rev()
        .map(|owned_path| {
            matcher::from_owned_path(&cwd, &owned_path.path, owned_path.owners.clone())
        })
        .collect::<Result<Vec<_>>>()?;
    let paths = stdin().lines().filter_map(|line| {
        let line = line.ok()?;
        if line.is_empty() {
            return None;
        }
        Some(line.to_string())
    });

    find_owners(owners.into_iter(), paths)?;

    Ok(())
}

fn find_owners<T: Iterator<Item = matcher::OwnedMatcher>, U: Iterator<Item = String>>(
    mut owners: T,
    paths: U,
) -> Result<()> {
    paths.for_each(|path| {
        let _ = owners
            .find(|owner| owner.ignorer.matched(&path, false).is_ignore())
            .map(|owner| println!("{}: {}", path, owner.owners.join(" ")));
    });

    Ok(())
}
