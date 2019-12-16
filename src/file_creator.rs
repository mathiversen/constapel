use crate::file_types;
use crate::result::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub type ConstantList = HashMap<String, HashMap<Value, Value>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileCreator {
    pub output_files: HashMap<String, OutputFiles>,
    pub constants: ConstantList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputFiles {
    pub path: String,
    #[serde(rename = "constants")]
    pub constants_ref: Vec<String>,
}

impl FileCreator {
    /// Initiate with a yaml string.
    pub fn from_yaml(content: String) -> Result<FileCreator> {
        serde_yaml::from_str(&content).map_err(Error::Yaml)
    }

    /// Initiate with a reference to a yaml file.
    pub fn from_yaml_file(path: &PathBuf) -> Result<FileCreator> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        serde_yaml::from_str(&content).map_err(Error::Yaml)
    }

    /// Run the program and create all constant files, according to the provideded structure.
    pub fn run(&self) -> Result<()> {
        for (file_ending, output_file) in self.output_files.iter() {
            let relevant_constants = self.get_relevant_constants(output_file);
            match file_ending.as_str() {
                "js" => file_types::js::create(&output_file.path, &relevant_constants, &self.constants)?,
                "scss" => file_types::scss::create(&output_file.path, &relevant_constants, &self.constants)?,
                "css" => file_types::css::create(&output_file.path, &relevant_constants, &self.constants)?,
                _ => return Err(Error::UnknownTarget(file_ending.clone())),
            }
        }

        Ok(())
    }

    pub fn get_relevant_constants(&self, output_file: &OutputFiles) -> ConstantList {
        self.constants.iter().fold(HashMap::new(), |mut acc, (key, value)| {
            if output_file.constants_ref.contains(&key) {
                acc.insert(key.clone(), value.clone());
                acc
            } else {
                acc
            }
        })
    }
}
