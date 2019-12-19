use crate::prelude::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs;

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


            if config.files == "many" {
                for constant_group_key in config.include.iter() {
                    let mut file = String::new();
                    if let Some(constants) = self.constants.get(constant_group_key) {
                        file.push_str(self.get_file_heading(&file_ending)?.as_str());
                        file.push_str(self.get_constant_object_start(&file_ending, match constant_group_key.as_str() {
                            "js" => Some(constant_group_key),
                            _ => None
                        })?.as_str());
                        for (index, (name, value)) in constants.iter().enumerate() {
                            file.push_str(self.get_formatted_constant(&file_ending, &constant_group_key, (name, value), index == constants.len() - 1)?.as_str())
                        }
                        file.push_str(self.get_constant_object_end(&file_ending)?)
                    } else {
                        return Err(Error::NotAConstant(constant_group_key.clone()));
                    }
                    self.create_file(&config.path, &constant_group_key, &file_ending, file)?
                }
            } else {
                println!("TODO ---- single!")
            }
        }
        Ok(())
    }

    fn create_file (&self, path: &String, file_name: &String, file_ending: &String, content: String) -> Result<()> {
        if fs::metadata(path).is_err() {
            fs::create_dir_all(path).expect("Failed to create directory");
        }
        let mut file = fs::File::create(format!("{}/{}.{}", path, file_name, file_ending)).expect("Failed to create file.");
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn get_file_heading(&self, file_ending: &str) -> Result<String> {
        match file_ending {
            "js" | "scss" => Ok(format!("// {}\n\n", &STR_DONT_EDIT)),
            "css" => Ok(format!("/* {} */\n\n", &STR_DONT_EDIT)),
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

    fn get_formatted_constant (&self, file_ending: &str, group_name: &str, constant_pair: (&Value, &Value), _is_last: bool) -> Result<String> {

        let key = constant_pair.0
            .as_str()
            .expect("Could not unwrap value");

        let string = match file_ending {
            "js" => format!("\t{}: {},\n", key, self.get_formatted_constant_value(file_ending, constant_pair.1)?),
            "css" => format!("\t--{}-{}: {};\n", group_name, key, self.get_formatted_constant_value(file_ending, constant_pair.1)?),
            "scss" => format!("${}-{}: {};\n", group_name, key, self.get_formatted_constant_value(file_ending, constant_pair.1)?),
            _ => panic!(Error::UnknownTarget(file_ending.to_owned()))
        };

        Ok(string)
    }

    fn get_formatted_constant_value (&self, file_ending: &str, value: &Value) -> Result<String> {

        // If value is reference, get that value instead
        let value = match &value {
            Value::String(v) => if v.matches("*").collect::<Vec<&str>>().len() > 0 {
                self.get_reference_value(v.clone())?
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

    fn get_reference_value(&self, reference: String) -> Result<&Value> {
        let mut keys: Vec<&str> = reference.split('.').collect();
        keys[0] = keys[0].trim_start_matches('*');

        if let Some(category) = self.constants.get(keys[0]) {
          if let Some(value) = category.get(&Value::String(keys[1].to_string())) {
            Ok(value)
          } else {
            Err(Error::FalsyReference(reference.to_string()))
          }
        } else {
            Err(Error::FalsyReference(reference.to_string()))
        }
    }
}