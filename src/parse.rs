use std::fs::{ File };
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::{Result, Error};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Constapel {
    pub output_files: HashMap<String, OutputFormats>,
    pub constants: ConstantList,
}

pub type ConstantList = HashMap<String, HashMap<Value, Value>>;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct OutputFormats {
    pub path: String,
    pub constants: Vec<String>,
}

impl Constapel {
  pub fn from_yaml (path: &PathBuf) -> Result<Constapel> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    serde_yaml::from_str(&contents).map_err(|e| Error::Yaml(e))
  }
}