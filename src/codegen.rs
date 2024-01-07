use std::collections::HashMap;

use genco::prelude::rust::Tokens;
use genco::prelude::*;
use itertools::Itertools;

use crate::prelude::*;
use crate::yaml::*;

struct InstructionInfo<'a> {
    name: &'a str,
    instruction: &'a Instruction,
    arguments: Vec<&'a str>,
    constants: HashMap<&'a str, &'a str>,
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
        let (_, arguments) = operands
            .syntax
            .split_once("%opcode")
            .context(anyhow!("invalid operand syntax: {:?}", operands.syntax))?;

        let arguments = arguments.trim().split(", ").collect::<Vec<&str>>();
        let registers = operands
            .inputs
            .iter()
            .chain(operands.outputs.iter())
            .map(|(k, v)| (k.as_ref(), v.as_ref()))
            .collect::<HashMap<&str, &str>>();
        let mut constants = HashMap::<&str, &str>::new();
        if !instruction.rd.is_empty() {
            constants.insert("rd", &instruction.rd);
        }
        if !instruction.rs.is_empty() {
            constants.insert("rs", &instruction.rs);
        }
        if !instruction.rt.is_empty() {
            constants.insert("rt", &instruction.rt);
        }

        Ok(InstructionInfo {
            name,
            instruction,
            arguments,
            constants,
            registers,
        })
    }

    fn prefix(&self, arg: &str) -> bool {
        self.arguments.contains(&arg)
            && match arg {
                "rd" => self.instruction.prefix.contains(['d', 'D']),
                "rs" => self.instruction.prefix.contains(['d', 'S']),
                "rt" => self.instruction.prefix.contains('t'),
                _ => false,
            }
    }

    fn arguments(&self) -> String {
        self.arguments
            .iter()
            .map(|arg| {
                if self.registers.contains_key(arg) || self.name.eq("vcst") {
                    if self.prefix(arg) {
                        format!("${arg}:ident $([$(${arg}p:tt)+])?")
                    } else {
                        format!("${arg}:ident")
                    }
                } else if self.name.eq("vrot") {
                    format!("[$(${arg}:tt)*]")
                } else {
                    format!("${arg}:expr")
                }
            })
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn constant(&self, arg: &str) -> Option<&&str> {
        self.constants.get(arg)
    }

    fn opcode(&self) -> String {
        format!(
            "0b{:0<32}",
            format!(
                "{}{}",
                self.instruction.opcode, self.instruction.opcode_suffix
            )
        )
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

fn vector_imm16(info: &InstructionInfo) -> Result<Tokens> {
    if info.instruction.flavors.is_empty() {
        return Err(anyhow!(
            "vector-imm16 instruction '{}' has no flavors",
            info.name
        ));
    }

    let mut tokens = Tokens::new();
    for flavor in info.instruction.flavors.chars() {
        let mode = match info.name {
            "viim" => Ok("0b0000000000000000"),
            "vfim" => Ok("0b0000000010000000"),
            _ => Err(anyhow!("unknown vector-imm16 instruction '{}'", info.name)),
        }?;
        tokens.append(quote! {
            $("\n")
            ($(format!("{}.{}", info.name, flavor)) $(info.arguments())) => {
                concat!(
                    $(if info.prefix("rd") { $$($$crate::instruction!(vpfxd $$($$rdp)*), "\n",)? })
                    $(quoted(format!(".word {}", info.opcode()))),
                    $(quoted(format!("| {}", mode))),
                    $(if info.arguments.contains(&"imm16") { "| ((", stringify!($$imm16), " & 0xFFFF) << 0)", })
                    $(if info.arguments.contains(&"rd") { "| (", $$crate::register_$(info.register("rd", flavor)?)!($$rd), " << 16)", })
                )
            };
        });
    }
    Ok(tokens)
}

fn vector_imm5(info: &InstructionInfo) -> Result<Tokens> {
    if info.instruction.flavors.is_empty() {
        return Err(anyhow!(
            "vector-imm5 instruction '{}' has no flavors",
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
            ($(format!("{}.{}", info.name, flavor)) $(info.arguments())) => {
                concat!(
                    $(if info.prefix("rd") { $$($$crate::instruction!(vpfxd $$($$rdp)*), "\n",)? })
                    $(if info.prefix("rs") { $$($$crate::instruction!(vpfxs $$($$rsp)*), "\n",)? })
                    $(quoted(format!(".word {}", info.opcode()))),
                    $(quoted(format!("| {}", mode))),
                    $(if info.arguments.contains(&"rd") { "| (", $$crate::register_$(info.register("rd", flavor)?)!($$rd), " << 0)", })
                    $(if let Some(constant) = info.constant("rd") { $(quoted(format!("| (0b{} << 0)", *constant))), })
                    $(if info.arguments.contains(&"rs") { "| (", $$crate::register_$(info.register("rs", flavor)?)!($$rs), " << 8)", })
                    $(if let Some(constant) = info.constant("rs") { $(quoted(format!("| (0b{} << 8)", *constant))), })
                    $(if info.arguments.contains(&"imm5") && info.name.eq("vcst") { "| (", $$crate::vfpu_const!($$imm5), " << 16)", })
                    $(if info.arguments.contains(&"imm5") && info.name.eq("vrot") { "| (", $$crate::vrot_immediate_$(info.register("rd", flavor)?)!($$($$imm5)*), " << 16)", })
                    $(if info.arguments.contains(&"imm5") && info.name.ne("vcst") && info.name.ne("vrot") { "| (", stringify!($$imm5), " << 16)", })
                    $(if info.arguments.contains(&"scale") { "| (", stringify!($$scale), " << 16)", })
                )
            };
        });
    }
    Ok(tokens)
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
            ($(format!("{}.{}", info.name, flavor)) $(info.arguments())) => {
                concat!(
                    $(if info.prefix("rd") { $$($$crate::instruction!(vpfxd $$($$rdp)*), "\n",)? })
                    $(if info.prefix("rs") { $$($$crate::instruction!(vpfxs $$($$rsp)*), "\n",)? })
                    $(if info.prefix("rt") { $$($$crate::instruction!(vpfxt $$($$rtp)*), "\n",)? })
                    $(quoted(format!(".word {}", info.opcode()))),
                    $(quoted(format!("| {}", mode))),
                    $(if info.arguments.contains(&"rd") { "| (", $$crate::register_$(info.register("rd", flavor)?)!($$rd), " << 0)", })
                    $(if let Some(constant) = info.constant("rd") { $(quoted(format!("| (0b{} << 0)", *constant))), })
                    $(if info.arguments.contains(&"rs") { "| (", $$crate::register_$(info.register("rs", flavor)?)!($$rs), " << 8)", })
                    $(if let Some(constant) = info.constant("rs") { $(quoted(format!("| (0b{} << 8)", *constant))), })
                    $(if info.arguments.contains(&"rt") { "| (", $$crate::register_$(info.register("rt", flavor)?)!($$rt), " << 16)", })
                    $(if let Some(constant) = info.constant("rt") { $(quoted(format!("| (0b{} << 16)", *constant))), })
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
            ($(format!("{}.{}", info.name, flavor)) $(info.arguments())) => {
                concat!(
                    $(if info.prefix("rd") { $$($$crate::instruction!(vpfxd $$($$rdp)*), "\n",)? })
                    $(if info.prefix("rs") { $$($$crate::instruction!(vpfxs $$($$rsp)*), "\n",)? })
                    $(if info.prefix("rt") { $$($$crate::instruction!(vpfxt $$($$rtp)*), "\n",)? })
                    $(quoted(format!(".word {}", info.opcode()))),
                    $(quoted(format!("| {}", mode))),
                    $(if info.arguments.contains(&"rd") { "| (", $$crate::register_$(info.register("rd", flavor)?)!($$rd), " << 0)", })
                    $(if info.arguments.contains(&"rd") { "| (", $$crate::register_$(info.register("rd", flavor)?)!($$rd), " << 0)", })
                    $(if info.arguments.contains(&"rs") { "| (", $$crate::register_$(info.register("rs", flavor)?)!($$rs), " << 8)", })
                    $(if info.arguments.contains(&"rt") { "| (", $$crate::register_$(info.register("rt", flavor)?)!($$rt), " << 16)", })
                )
            };
        });
    }
    Ok(tokens)
}

fn vfpu_fixedop(info: &InstructionInfo) -> Result<Tokens> {
    let mut tokens = Tokens::new();
    tokens.append(quote! {
        $("\n")
        ($(format!("{}", info.name))) => {
            $(quoted(format!(".word {}", info.opcode())))
        };
    });
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
        "vector-imm16" => Some(vector_imm16(&info)?),
        "vector-imm5" => Some(vector_imm5(&info)?),
        "vfpu-alu-m1" => Some(vfpu_alu_m1(&info)?),
        "vfpu-alu" => Some(vfpu_alu(&info)?),
        "vfpu-fixedop" => Some(vfpu_fixedop(&info)?),
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
