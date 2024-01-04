use std::io::Write;

use prelude::*;

mod codegen;
mod prelude;
mod yaml;

fn main() -> Result<()> {
    let file: yaml::File = serde_yaml::from_str(include_str!("../res/inst-vfpu-desc.yaml"))?;
    let mut output = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .read(false)
        .open("generated.rs")
        .context("failed to open file")?;
    for (name, instruction) in &file.instructions {
        if let Some(code) = codegen::write_instruction(name, instruction, &file)? {
            output
                .write(code.as_bytes())
                .context("failed to write to file")?;
        }
    }
    output.flush().context("failed to flush file")?;
    Ok(())
}
