use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use crate::result::{Result, Error};
use crate::file_types;

pub type ConstantList = HashMap<String, HashMap<Value, Value>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileCreator {
    pub output_files: HashMap<String, FileFormats>,
    pub constants: ConstantList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileFormats {
    pub path: String,
    pub constants: Vec<String>,
}

impl FileCreator {
    pub fn from_yaml(path: &PathBuf) -> Result<FileCreator> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        serde_yaml::from_str(&contents).map_err(Error::Yaml)
    }

    pub fn run(self) -> Result<()> {
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

        Ok(())
    }
}