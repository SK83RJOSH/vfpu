use std::io::Write;

use prelude::*;

mod codegen;
mod prelude;
mod yaml;

fn main() -> Result<()> {
    let database: yaml::Database =
        serde_yaml::from_str(include_str!("../res/inst-vfpu-desc.yaml"))?;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .read(false)
        .open("generated.rs")
        .context("failed to open file")?;
    let document = codegen::generate_document(&database)?;
    file.write(document.trim_end().as_bytes())
        .context("failed to write to file")?;
    Ok(())
}
