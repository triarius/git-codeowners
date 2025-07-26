use color_eyre::Result;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::collections::HashMap;

use crate::parser::CodeOwners;

pub fn find_and_print(codeowners: CodeOwners, paths: impl Iterator<Item = String>) -> Result<()> {
    let codeowners_by_glob = codeowners
        .clone()
        .into_iter()
        .map(|owned_path| (owned_path.path.clone(), owned_path.owners))
        .collect::<HashMap<String, Vec<String>>>();

    let mut builder = GitignoreBuilder::new(std::env::current_dir()?);
    let builder = codeowners
        .into_iter()
        .try_fold(&mut builder, |acc, op| acc.add_line(None, &op.path))?;

    find(builder.build()?, codeowners_by_glob, paths).for_each(|(path, owners)| {
        if owners.is_empty() {
            println!("{path}: No owners found");
        } else {
            println!("{path}: {}", owners.join(", "));
        }
    });

    Ok(())
}

fn find(
    matcher: Gitignore,
    owners_by_glob: HashMap<String, Vec<String>>,
    paths: impl Iterator<Item = String>,
) -> impl Iterator<Item = (String, Vec<String>)> {
    paths.map(move |path| match matcher.matched(&path, false) {
        ignore::Match::Ignore(glob) => {
            if let Some(owners) = owners_by_glob.get(glob.original()) {
                (path, owners.clone())
            } else {
                (path, vec![])
            }
        }
        _ => (path, vec![]),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ignore::gitignore::GitignoreBuilder;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_find() {
        let matcher = GitignoreBuilder::new(".")
            .add_line(None, "src/*.rs")
            .unwrap()
            .build()
            .unwrap();

        let owners_by_glob = HashMap::from([(
            "src/*.rs".to_string(),
            vec!["Alice".to_string(), "Bob".to_string()],
        )]);

        let paths = vec![
            "src/main.rs".to_string(),
            "src/lib.rs".to_string(),
            "README.md".to_string(),
        ];

        let results: Vec<_> = find(matcher, owners_by_glob, paths.into_iter()).collect();

        let expected = vec![
            (
                "src/main.rs".to_string(),
                vec!["Alice".to_string(), "Bob".to_string()],
            ),
            (
                "src/lib.rs".to_string(),
                vec!["Alice".to_string(), "Bob".to_string()],
            ),
            ("README.md".to_string(), vec![]),
        ];

        assert_eq!(results, expected);
    }
}
