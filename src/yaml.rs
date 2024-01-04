use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Immediate {
    #[serde(rename = "minval")]
    pub min: usize,
    #[serde(rename = "maxval")]
    pub max: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Operand {
    pub syntax: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub inputs: HashMap<String, String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub outputs: HashMap<String, String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub immediates: HashMap<String, Immediate>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Encoding {
    pub encoding: String,
    pub fields: HashMap<char, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Instruction {
    #[serde(rename = "type")]
    pub operands: String,
    pub encoding: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub flavors: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub opcode: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub prefix: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    #[serde(rename = "instruction-operands")]
    pub operands: HashMap<String, Operand>,
    pub encodings: HashMap<String, Encoding>,
    pub instructions: HashMap<String, Instruction>,
}
