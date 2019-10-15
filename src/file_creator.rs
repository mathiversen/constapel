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

    /// This method parses a file using the reference of its path as the argument. It will return an error
    /// unless the file is formatted according to the FileCreator struct.
    pub fn from_yaml(content: String) -> Result<FileCreator> {
        serde_yaml::from_str(&content).map_err(Error::Yaml)
    }

    /// This method parses a file using the reference of its path as the argument. It will return an error
    /// unless the file is formatted according to the FileCreator struct.
    pub fn from_yaml_file(path: &PathBuf) -> Result<FileCreator> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        serde_yaml::from_str(&content).map_err(Error::Yaml)
    }

    /// This is the main method, used to match against provided file endings
    /// and then create files for each match.
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

mod tests {
    #![allow(unused_imports)]
    use super::*;
    use unindent::unindent;

    #[test]
    fn it_can_parse_yaml() {
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
        let c = FileCreator::from_yaml(yaml.to_string()).expect("Failed to parse.");
        assert_eq!(&c.output_files["js"].path, ".");
        assert_eq!(&c.constants["colors"][&serde_yaml::Value::String("white".to_string())], "#ffffff");
    }
}
