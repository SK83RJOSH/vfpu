use std::collections::HashMap;

use genco::prelude::rust::Tokens;
use genco::prelude::*;
use itertools::Itertools;

use crate::prelude::*;
use crate::yaml::*;

struct InstructionInfo<'a> {
    name: &'a str,
    instruction: &'a Instruction,
    args: Vec<&'a str>,
    registers: HashMap<&'a str, &'a str>,
}

impl InstructionInfo<'_> {
    fn new<'a>(
        name: &'a str,
        instruction: &'a Instruction,
        database: &'a Database,
    ) -> Result<InstructionInfo<'a>> {
        let operands = database
            .operands
            .get(&instruction.operands)
            .context(anyhow!(
                "failed to find operands: {:?}",
                instruction.operands
            ))?;
        let (_, args) = operands
            .syntax
            .split_once("%opcode")
            .context(anyhow!("invalid operand syntax: {:?}", operands.syntax))?;

        let args = args.trim().split(", ").collect::<Vec<&str>>();
        let registers = operands
            .inputs
            .iter()
            .chain(operands.outputs.iter())
            .map(|(k, v)| (k.as_ref(), v.as_ref()))
            .collect::<HashMap<&str, &str>>();

        Ok(InstructionInfo {
            name,
            instruction,
            args,
            registers,
        })
    }

    fn idents(&self) -> String {
        self.args
            .iter()
            .map(|arg| format!("${arg}:ident"))
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn opcode(&self) -> String {
        format!("0b{:0<32}", self.instruction.opcode)
    }

    fn register(&self, arg: &str, flavor: char) -> Result<&str> {
        let matrix = |flavor: char| -> Result<&str> {
            match flavor {
                's' => Ok("quad"),
                'p' => Ok("mpair"),
                't' => Ok("mtriple"),
                'q' => Ok("mquad"),
                _ => Err(anyhow!("register flavor '{flavor}' cannot be matrix")),
            }
        };
        let vector = |flavor: char| -> Result<&str> {
            match flavor {
                's' => Ok("single"),
                'p' => Ok("pair"),
                't' => Ok("triple"),
                'q' => Ok("quad"),
                _ => Err(anyhow!("register flavor '{flavor}' cannot be vector")),
            }
        };
        let vectorh = |flavor: char| -> Result<&str> {
            match flavor {
                'p' => Ok("single"),
                'q' => Ok("pair"),
                _ => Err(anyhow!("register flavor '{flavor}' cannot be vectorh")),
            }
        };
        let vectord = |flavor: char| -> Result<&str> {
            match flavor {
                's' => Ok("pair"),
                'p' => Ok("quad"),
                _ => Err(anyhow!("register flavor '{flavor}' cannot be vectord")),
            }
        };
        let vectorq = |flavor: char| -> Result<&str> {
            match flavor {
                's' => Ok("quad"),
                _ => Err(anyhow!("register flavor '{flavor}' cannot be vectorq")),
            }
        };
        if let Some(register) = self.registers.get(arg) {
            match *register {
                "matrix" => Ok(matrix(flavor)?),
                "vector" => Ok(vector(flavor)?),
                "vector:H" => Ok(vectorh(flavor)?),
                "vector:D" => Ok(vectord(flavor)?),
                "vector:Q" => Ok(vectorq(flavor)?),
                "single" => Ok("single"),
                _ => Err(anyhow!("unknown register type '{register}'")),
            }
        } else {
            Err(anyhow!("failed to find register type for '{arg}'"))
        }
    }
}

fn vfpu_alu(info: &InstructionInfo) -> Result<Tokens> {
    if info.instruction.flavors.is_empty() {
        return Err(anyhow!(
            "vfpu-alu instruction '{}' has no flavors",
            info.name
        ));
    }

    let mut tokens = Tokens::new();
    for flavor in info.instruction.flavors.chars() {
        let mode = match flavor {
            's' => Ok("0b0000000000000000"),
            'p' => Ok("0b0000000010000000"),
            't' => Ok("0b1000000000000000"),
            'q' => Ok("0b1000000010000000"),
            _ => Err(anyhow!("unknown flavor: {flavor}")),
        }?;
        tokens.append(quote! {
            $("\n")
            ($(format!("{}.{}", info.name, flavor)) $(info.idents())) => {
                concat!(
                    $(quoted(format!(".word {}", info.opcode()))),
                    $(quoted(format!("| {}", mode))),
                    $(if info.args.contains(&"rd") { "| (", $$crate::register_$(info.register("rd", flavor)?)!($$rd), " << 0)", })
                    $(if info.args.contains(&"rs") { "| (", $$crate::register_$(info.register("rs", flavor)?)!($$rs), " << 8)", })
                    $(if info.args.contains(&"rt") { "| (", $$crate::register_$(info.register("rt", flavor)?)!($$rt), " << 16)", })
                )
            };
        });
    }
    Ok(tokens)
}

fn vfpu_alu_m1(info: &InstructionInfo) -> Result<Tokens> {
    if info.instruction.flavors.is_empty() {
        return Err(anyhow!(
            "vfpu-alu-m1 instruction '{}' has no flavors",
            info.name
        ));
    }

    let mut tokens = Tokens::new();
    for flavor in info.instruction.flavors.chars() {
        let mode = match flavor {
            'p' => Ok("0b0000000000000000"),
            't' => Ok("0b0000000010000000"),
            'q' => Ok("0b1000000000000000"),
            _ => Err(anyhow!("unknown flavor: {flavor}")),
        }?;
        tokens.append(quote! {
            $("\n")
            ($(format!("{}.{}", info.name, flavor)) $(info.idents())) => {
                concat!(
                    $(quoted(format!(".word {}", info.opcode()))),
                    $(quoted(format!("| {}", mode))),
                    $(if info.args.contains(&"rd") { "| (", $$crate::register_$(info.register("rd", flavor)?)!($$rd), " << 0)", })
                    $(if info.args.contains(&"rs") { "| (", $$crate::register_$(info.register("rs", flavor)?)!($$rs), " << 8)", })
                    $(if info.args.contains(&"rt") { "| (", $$crate::register_$(info.register("rt", flavor)?)!($$rt), " << 16)", })
                )
            };
        });
    }
    Ok(tokens)
}

fn write_instruction(
    name: &str,
    instruction: &Instruction,
    database: &Database,
    tokens: &mut Tokens,
) -> Result<()> {
    let info = InstructionInfo::new(name, instruction, database)?;
    if let Some(instruction_tokens) = match instruction.encoding.as_str() {
        "vfpu-alu" => Some(vfpu_alu(&info)?),
        "vfpu-alu-m1" => Some(vfpu_alu_m1(&info)?),
        _ => None,
    } {
        for line in info.instruction.description.trim().split('\n') {
            tokens.append(quote! {
                $(format!("// {}", line)) $(&instruction_tokens)
                $("\n")
            });
        }
    }
    Ok(())
}

pub fn generate_document(database: &Database) -> Result<String> {
    let mut tokens = Tokens::new();
    for (name, instruction) in database.instructions.iter().sorted_by_key(|v| v.0) {
        write_instruction(name, instruction, database, &mut tokens)?;
    }
    tokens
        .to_file_string()
        .context("failed to generate document")
}
