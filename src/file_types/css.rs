use super::STR_DONT_EDIT;
use crate::{file_creator::ConstantList, result::{Result, Error}};
use serde_yaml::Value;
use std::fs;
use std::io::Write;

pub fn create(dir_path: &str, constants: &ConstantList) -> Result<()> {

      // Create dir
    if fs::metadata(dir_path).is_err() {
        fs::create_dir_all(dir_path).expect("Failed to create directory");
    }

    let mut file = fs::File::create(format!("{}/root.css", dir_path)).expect("Failed to create file.");
    let mut file_content = String::new();

    file_content.push_str(format!("/* {} */\n\n", STR_DONT_EDIT).as_str());
    file_content.push_str(":root {\n");

    for (constant_group, constant) in constants.iter() {

        for (key, value) in constant.iter() {
            match (key, value) {
                // String
                (Value::String(s1), Value::String(s2)) => {
                     file_content.push_str(format!("{:4}--{}-{}: '{}';\n", "", constant_group, s1, s2).as_str())
                }
                // Number
                (Value::String(s), Value::Number(n)) => {
                    file_content.push_str(format!("{:4}--{}-{}: {};\n", "", constant_group, s, n).as_str())
                }
                x => return Err(Error::NotSupportedValue(format!("{:?} and {:?}", x.0, x.1)))
            }
        }
    }
    file_content.push_str("}\n");
    file.write_all(file_content.as_bytes())?;

    Ok(())
}