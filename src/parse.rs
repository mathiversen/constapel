use crate::file_types;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub type ConstantList = HashMap<String, HashMap<Value, Value>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Constapel {
    pub output_files: HashMap<String, OutputFormats>,
    pub constants: ConstantList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputFormats {
    pub path: String,
    pub constants: Vec<String>,
}

impl Constapel {
    pub fn from_yaml(path: &PathBuf) -> Result<Constapel> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        serde_yaml::from_str(&contents).map_err(|e| Error::Yaml(e))
    }

    pub fn run(self) -> Result<Self> {
        for (file_ending, output_file) in self.output_files.iter() {
            let relevant_constants: ConstantList =
                self.constants
                    .iter()
                    .fold(HashMap::new(), |mut acc, (key, value)| {
                        if output_file.constants.contains(&key) {
                            acc.insert(key.clone(), value.clone());
                            acc
                        } else {
                            acc
                        }
                    });
            match file_ending.as_str() {
                "js" => file_types::js::create(&output_file.path, &relevant_constants)?,
                "scss" => file_types::scss::create(&output_file.path, &relevant_constants)?,
                _ => return Err(Error::UnknownTarget(file_ending.clone())),
            }
        }

        Ok(self)
    }
}
