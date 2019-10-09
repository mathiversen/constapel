use std::io::Write;
use std::fs;
use serde_yaml::Value;
use crate::{Constants, STR_DONT_EDIT, Result};

pub fn create(dir_path: &str, constants: &Constants) -> Result<()> {
    // Create dir
    if fs::metadata(dir_path).is_err() {
        fs::create_dir_all(dir_path).expect("Failed to create directory");
    }

    // Write to file
    for (constant_group, constant) in constants.iter() {
        let mut file = fs::File::create(format!("{}/{}.js", dir_path, constant_group)).expect("Failed to create file.");
        file.write_all(format!("// {}\n\n", STR_DONT_EDIT).as_bytes())?;
        file.write_all("export default {\n".as_bytes())?;
        for (index, (key, value)) in constant.iter().enumerate() {

            match (key, value) {
                (Value::String(s1), Value::String(s2)) => file.write_all(format!("{:4}{}: '{}'", "", s1, s2).as_bytes())?,
                (Value::String(s1), Value::Number(s2)) => file.write_all(format!("{:4}{}: {}", "", s1, s2).as_bytes())?,
                _ => unimplemented!()
            }

            if index == constant.len() - 1 {
                file.write_all(format!("\n").as_bytes())?
            } else {
                file.write_all(format!(",\n").as_bytes())?
            }

        }
        file.write_all("}\n".as_bytes())?;
    }

    Ok(())
}