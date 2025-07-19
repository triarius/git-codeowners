use color_eyre::Result;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

#[derive(Debug)]
pub struct OwnedMatcher {
    pub owners: Vec<String>,
    pub ignorer: Gitignore,
}

pub fn from_owned_path<P: AsRef<Path>>(
    cwd: P,
    glob: &str,
    owners: Vec<String>,
) -> Result<OwnedMatcher> {
    let ignorer = GitignoreBuilder::new(&cwd).add_line(None, glob)?.build()?;
    Ok(OwnedMatcher { owners, ignorer })
}
