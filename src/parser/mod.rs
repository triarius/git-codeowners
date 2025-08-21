#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OwnedPath {
    pub path: String,
    pub owners: Vec<String>,
}

pub type CodeOwners = Vec<OwnedPath>;

pub fn parse(input: &str) -> CodeOwners {
    input
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| {
            // Remove suffix that begins with `#` and trim whitespace
            let line = line.split('#').next().unwrap_or("").trim();

            let mut tokens = line.split_whitespace();

            let path = tokens.next()?.to_string();
            let owners: Vec<String> = tokens.map(String::from).collect();

            let path = if path.ends_with('/') {
                format!("{path}**")
            } else {
                path
            };

            Some(OwnedPath { path, owners })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let input = r"
            # This is a comment
            /src/ codeowner1 codeowner2
            /docs/ codeowner3
            # Another comment
            /tests/ codeowner4
            /src/utils/ codeowner5
            # Final comment
            /src/utils/helpers/ codeowner6
            ";

        let expected: CodeOwners = vec![
            ("/src/**", vec!["codeowner1", "codeowner2"]),
            ("/docs/**", vec!["codeowner3"]),
            ("/tests/**", vec!["codeowner4"]),
            ("/src/utils/**", vec!["codeowner5"]),
            ("/src/utils/helpers/**", vec!["codeowner6"]),
        ]
        .into_iter()
        .map(|(path, owners)| OwnedPath {
            path: path.to_string(),
            owners: owners.iter().map(|&s| s.to_string()).collect(),
        })
        .collect();

        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_empty() {
        let input = "";
        let expected: CodeOwners = vec![];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_example() {
        let input = std::fs::read_to_string("src/parser/examples/1.CODEOWNERS")
            .expect("Failed to read example file");

        let expected: CodeOwners = vec![
            ("*", vec!["@global-owner1", "@global-owner2"]),
            ("*.js", vec!["@js-owner"]),
            ("*.go", vec!["docs@example.com"]),
            ("*.txt", vec!["@octo-org/octocats"]),
            ("/build/logs/**", vec!["@doctocat"]),
            ("docs/*", vec!["docs@example.com"]),
            ("apps/**", vec!["@octocat"]),
            ("/docs/**", vec!["@doctocat"]),
            ("/scripts/**", vec!["@doctocat", "@octocat"]),
            ("**/logs", vec!["@octocat"]),
            ("/apps/**", vec!["@octocat"]),
            ("/apps/github", vec![]),
            ("/apps/**", vec!["@octocat"]),
            ("/apps/github", vec!["@doctocat"]),
        ]
        .into_iter()
        .map(|(path, owners)| OwnedPath {
            path: path.to_string(),
            owners: owners.iter().map(|&s| s.to_string()).collect(),
        })
        .collect();

        assert_eq!(parse(&input), expected);
    }
}
