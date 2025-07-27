use color_eyre::Result;
use ignore::gitignore::GitignoreBuilder;

use crate::parser::CodeOwners;

pub fn by(codeowners: CodeOwners, owner: &str, dir: &str) -> Result<()> {
    let mut builder = GitignoreBuilder::new(std::env::current_dir()?);
    let owned_files = codeowners
        .into_iter()
        .filter(|op| op.owners.iter().any(|o| o == owner))
        .try_fold(&mut builder, |acc, op| acc.add_line(None, &op.path))?
        .build()?;

    ignore::Walk::new(dir)
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path().to_string_lossy().to_string();

            if owned_files.matched(&path, false).is_ignore() {
                Some(path)
            } else {
                None
            }
        })
        .for_each(|path| println!("{path}"));

    Ok(())
}
