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
    pub output_files: HashMap<String, FileFormats>,
    pub constants: ConstantList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileFormats {
    pub path: String,
    pub constants: Vec<String>,
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
            let relevant_constants: ConstantList = self.constants.iter().fold(HashMap::new(), |mut acc, (key, value)| {
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
                "css" => file_types::css::create(&output_file.path, &relevant_constants)?,
                _ => return Err(Error::UnknownTarget(file_ending.clone())),
            }
        }

        Ok(())
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::*;
    use unindent::unindent;

    #[test]
    fn parse_yaml_string() {
        let yaml = r#"
          output_files:
            js:
              path: '.'
              constants:
                - colors
          constants:
            colors:
              white: '#ffffff'
        "#;
        let c = FileCreator::from_yaml(yaml.to_string()).unwrap();
        assert_eq!(&c.output_files["js"].path, ".");
        assert_eq!(
            &c.constants["colors"][&serde_yaml::Value::String("white".to_string())],
            "#ffffff"
        );
    }
}
