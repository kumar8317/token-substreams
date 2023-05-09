use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("Ownable", "abi/ownable.json")?
        .generate()?
        .write_to_file("src/abi/ownable.rs")?;

    Ok(())
}
