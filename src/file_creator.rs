use crate::utils::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

const STR_DONT_EDIT: &str = "DON'T EDIT THIS FILE - IT'S GENERATED";

pub type Constants = HashMap<String, HashMap<Value, Value>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub path: String,
    pub include: Vec<String>,
    pub files: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Constapel {
    pub config: HashMap<String, Config>,
    pub constants: Constants,
}

impl Constapel {
    /// Initiate with a yaml-string.
    pub fn from_yaml(content: String) -> Result<Constapel> {
        serde_yaml::from_str(&content).map_err(Error::Yaml)
    }

    /// Initiate with a reference to a yaml-file.
    pub fn from_yaml_file(path: &PathBuf) -> Result<Constapel> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        serde_yaml::from_str(&content).map_err(Error::Yaml)
    }

    /// Run the program and create all constant files, according to the provideded structure.
    pub fn run(&self) -> Result<()> {
        for (file_ending, config) in self.config.iter() {

            let mut files: Vec<String> = vec![];

            if config.files == "many" {
                for constant_group_key in config.include.iter() {
                    let file = String::new();
                    if let Some(foo) = self.constants.get(constant_group_key) {
                        file.push_str(self.get_file_heading(&file_ending)?);
                        file.push_str(self.get_constant_object_start(&file_ending, match constant_group_key.as_str() {
                            "js" => Some(constant_group_key),
                            _ => None
                        })?.as_str());
                        // get_formatted_constant
                        // get_constant_object_end
                    } else {
                        Err(Error::NotAConstant(*constant_group_key));
                    }
                    files.push(file);
                }
            } else {
                let file = String::new();
                // get_file_heading
                // get_constant_object_start
                // get_formatted_constant
                // get_constant_object_end
                files.push(file);
            }
            // for file in files write to path
        }
        Ok(())
    }

    fn get_file_heading(&self, file_ending: &str) -> Result<&str> {
        match file_ending {
            "js" | "scss" => Ok(format!("// {}\n\n", &STR_DONT_EDIT).as_str()),
            "css" => Ok(format!("/* {} */\n\n", &STR_DONT_EDIT).as_str()),
            _ => Err(Error::UnknownTarget(file_ending.to_owned()))
        }
    }

    fn get_constant_object_start <'a> (&self, file_ending: &str, name: Option<&'a str>) -> Result<String> {
        match file_ending {
            "js" => Ok(format!("export {} {{\n", self.get_constant_object_name(file_ending, name)?)),
            "css" => Ok(format!("{} {{\n", self.get_constant_object_name(file_ending, name)?)),
            "scss" => Ok(format!("{}", self.get_constant_object_name(file_ending, name)?)),
            _ => Err(Error::UnknownTarget(file_ending.to_owned()))
        }
    }

    fn get_constant_object_name <'a> (&self, file_ending: &str, name: Option<&'a str>) -> Result<&'a str> {
        if name.is_some() {
            Ok(name.unwrap())
        } else {
            match file_ending {
                "js" => Ok("default"),
                "css" => Ok(":root"),
                "scss" => Ok(""),
                _ => Err(Error::UnknownTarget(file_ending.to_owned()))
            }
        }
    }

    fn get_constant_object_end <'a> (&self, file_ending: &str) -> Result<&str> {
        match file_ending {
            "js" | "css" => Ok("}"),
            "scss" => Ok(""),
            _ => Err(Error::UnknownTarget(file_ending.to_owned()))
        }
    }

    fn get_formatted_constant (&self, file_ending: &str, constant_pair: (Value, Value), is_last: bool) -> Result<String> {

        let key = constant_pair.1
            .as_str()
            .unwrap_or_else(|| {
                panic!(Err(Error::NotSupportedValue(format!("{:?}", constant_pair.0))))
            });

        let string = match file_ending {
            "js" | "css" | "scss" => Ok(format!("\t{}: {}", key, self.get_formatted_constant_value(file_ending, constant_pair.1)?)),
            _ => Err(Error::UnknownTarget(file_ending.to_owned()))
        };

        if is_last {
            string?.push(',');
        }
        string
    }

    fn get_formatted_constant_value (&self, file_ending: &str, value: Value) -> Result<String> {

        // If value is reference, get that value instead
        let value = match value {
            Value::String(v) => if v.matches("*").collect::<Vec<&str>>().len() > 0 {
                self.get_reference_value(&v)?
            } else {
                value
            }
            _ => value
        };

        match file_ending {
            "js" | "css" | "scss" => match value {
                Value::String(v) => Ok(format!("'{}'", v)),
                Value::Number(v) => Ok(format!("{}", v)),
                _ => Err(Error::NotSupportedValue(format!("{:?}", value)))
            },
            _ => Err(Error::UnknownTarget(file_ending.to_owned()))
        }
    }

    fn get_reference_value(&self, reference: &'static str) -> Result<Value> {
        let mut keys: Vec<&str> = reference.split('.').collect();
        keys[0] = keys[0].trim_start_matches('*');

        if let Some(category) = self.constants.get(keys[0]) {
          if let Some(value) = category.get(&Value::String(keys[1].to_string())) {
            Ok(value.clone())
          } else {
            Err(Error::FalsyReference(reference.to_string()))
          }
        } else {
            Err(Error::FalsyReference(reference.to_string()))
        }
    }
}
