use std::collections::HashMap;

use genco::prelude::*;

use crate::prelude::*;
use crate::yaml::*;

fn vfpu_alu(name: &str, instruction: &Instruction, database: &File) -> Result<String> {
    if instruction.flavors.is_empty() {
        return Err(anyhow!("vfpu-alu instruction has no flavors"));
    }

    let operands = database
        .operands
        .get(&instruction.operands)
        .context(anyhow!(
            "failed to find operands: {:?}",
            instruction.operands
        ))?;

    let (_, args) = operands
        .syntax
        .split_once(' ')
        .context(anyhow!("invalid operand syntax: {:?}", operands.syntax))?;

    let args = args.split(", ").collect::<Vec<&str>>();
    let registers = operands
        .inputs
        .clone()
        .into_iter()
        .chain(operands.outputs.clone())
        .collect::<HashMap<String, String>>();
    let opcode = format!("0b{:0<32}", instruction.opcode);
    let idents = args
        .iter()
        .map(|arg| format!("${arg}:ident"))
        .collect::<Vec<String>>()
        .join(", ");

    let mut tokens: rust::Tokens = rust::Tokens::new();

    for line in instruction.description.trim().split('\n') {
        tokens.append(quote! {
            $(format!("// {}", line))
        });
    }

    let matrix = |register_type: &str| -> Result<&str> {
        match register_type {
            "single" => Ok("quad"),
            "pair" => Ok("mpair"),
            "triple" => Ok("mtriple"),
            "quad" => Ok("mquad"),
            _ => Err(anyhow!("register type '{register_type}' cannot be matrix")),
        }
    };
    let half = |register_type: &str| -> Result<&str> {
        match register_type {
            "pair" => Ok("single"),
            "quad" => Ok("pair"),
            _ => Err(anyhow!("register type '{register_type}' cannot be halved")),
        }
    };
    let double = |register_type: &str| -> Result<&str> {
        match register_type {
            "single" => Ok("pair"),
            "pair" => Ok("quad"),
            _ => Err(anyhow!("register type '{register_type}' cannot be doubled")),
        }
    };
    let quadruple = |register_type: &str| -> Result<&str> {
        match register_type {
            "single" => Ok("quad"),
            _ => Err(anyhow!(
                "register type '{register_type}' cannot be quadrupled"
            )),
        }
    };

    for flavor in instruction.flavors.chars() {
        let (mode, register) = match flavor {
            's' => Ok(("0b0000000000000000", "single")),
            'p' => Ok(("0b0000000010000000", "pair")),
            't' => Ok(("0b1000000000000000", "triple")),
            'q' => Ok(("0b1000000010000000", "quad")),
            _ => Err(anyhow!("unknown flavor: {flavor}")),
        }?;

        let get_register = |arg: &str| -> Result<&str> {
            if let Some(register_type) = &registers.get(arg) {
                match register_type.as_str() {
                    "vector" => Ok(register),
                    "matrix" => Ok(matrix(register)?),
                    "vector:H" => Ok(half(register)?),
                    "vector:D" => Ok(double(register)?),
                    "vector:Q" => Ok(quadruple(register)?),
                    "single" => Ok("single"),
                    _ => Err(anyhow!("unknown register type '{register_type}'")),
                }
            } else {
                Err(anyhow!("failed to find register type for '{arg}'"))
            }
        };

        tokens.append(quote! {
            $("\n")
            ($(format!("{name}.{flavor}")) $(&idents)) => {
                concat!(
                    $(quoted(format!(".word {}", opcode))),
                    $(quoted(format!("| {}", mode))),
                    $(if args.contains(&"rd") { "| (", $$crate::register_$(get_register("rd")?)!($$rd), " << 0)", })
                    $(if args.contains(&"rs") { "| (", $$crate::register_$(get_register("rs")?)!($$rs), " << 8)", })
                    $(if args.contains(&"rt") { "| (", $$crate::register_$(get_register("rt")?)!($$rt), " << 16)", })
                )
            };
        });
    }

    tokens.append(quote! {
        $("\n")
    });

    Ok(tokens.to_file_string()?)
}

pub fn write_instruction(
    name: &str,
    instruction: &Instruction,
    database: &File,
) -> Result<Option<String>> {
    match instruction.encoding.as_str() {
        "vfpu-alu" => Ok(Some(vfpu_alu(name, instruction, database)?)),
        _ => Ok(None),
    }
}
