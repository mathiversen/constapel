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

    // Write to file
    for (constant_group, constant) in constants.iter() {

        let mut file = fs::File::create(format!("{}/{}.scss", dir_path, constant_group)).expect("Failed to create file.");
        let mut file_content = String::new();

        file_content.push_str(format!("// {}\n\n", STR_DONT_EDIT).as_str());

        for (key, value) in constant.iter() {
            match (key, value) {
                (Value::String(s1), Value::String(s2)) => {
                     file_content.push_str(format!("${}-{}: '{}';\n", constant_group, s1, s2).as_str())
                }
                (Value::String(s1), Value::Number(s2)) => {
                    file_content.push_str(format!("${}-{}: {};\n", constant_group, s1, s2).as_str())
                }
                x => return Err(Error::UnknownValueType(format!("{:?} and {:?}", x.0, x.1)))
            }
        }
        file.write_all(file_content.as_bytes())?;
    }

    Ok(())
}
