mod parser;

use color_eyre::Result;

fn main() -> Result<()> {
    let codeowner = std::fs::read_to_string(".github/CODEOWNERS")?;
    let parsed = parser::parse(&codeowner);

    println!("{parsed:#?}");

    Ok(())
}
