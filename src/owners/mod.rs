use ignore::{gitignore::Gitignore, Match};
use std::collections::HashMap;

pub fn find<I: Iterator<Item = String>>(
    matcher: Gitignore,
    owners_by_glob: HashMap<String, Vec<String>>,
    paths: I,
) -> impl Iterator<Item = (String, Vec<String>)> {
    paths.map(move |path| match matcher.matched(&path, false) {
        Match::Ignore(glob) => {
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
