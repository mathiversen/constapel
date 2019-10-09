use std::fs::{ File };
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::Result;

pub type Constants = HashMap<String, HashMap<Value, Value>>;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Inputfile {
    pub output_files: HashMap<String, Outputfile>,
    pub constants: Constants,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Outputfile {
    pub path: String,
    pub constants: Vec<String>,
}

// TODO: Replace with impl?
pub fn serialize_input_file(input: &PathBuf) -> Result<Inputfile> {
    let mut file = File::open(input)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let x = serde_yaml::from_str(&contents).expect("Failed to parse yaml file");
    Ok(x)
}