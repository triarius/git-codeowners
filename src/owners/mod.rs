use crate::parser;
use ignore::{gitignore::Gitignore, Match};
use std::collections::HashMap;

pub fn find<I: Iterator<Item = String>>(
    matcher: Gitignore,
    owners_by_glob: HashMap<String, parser::OwnedPath>,
    paths: I,
) -> impl Iterator<Item = (String, Vec<String>)> {
    paths.map(move |path| match matcher.matched(&path, false) {
        Match::Ignore(glob) => {
            if let Some(owner) = owners_by_glob.get(glob.original()) {
                (path, owner.owners.clone())
            } else {
                (path, vec![])
            }
        }
        _ => (path, vec![]),
    })
}
