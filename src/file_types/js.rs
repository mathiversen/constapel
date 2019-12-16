use super::STR_DONT_EDIT;
use crate::{
    file_creator::ConstantList,
    result::{Error, Result},
};
use serde_yaml::Value;
use std::fs;
use std::io::Write;

pub fn create(dir_path: &str, constants: &ConstantList, all_constants: &ConstantList) -> Result<()> {
    // Create dir
    if fs::metadata(dir_path).is_err() {
        fs::create_dir_all(dir_path).expect("Failed to create directory");
    }

    // Write to file
    for (constant_group, constant) in constants.iter() {
        let mut file = fs::File::create(format!("{}/{}.js", dir_path, constant_group)).expect("Failed to create file.");
        let mut file_content = String::new();

        file_content.push_str(format!("// {}\n\n", STR_DONT_EDIT).as_str());
        file_content.push_str("export default {\n");

        for (index, (key, value)) in constant.iter().enumerate() {
            match (key, value) {
                (Value::String(key), Value::String(value)) => match &value.chars().next() {
                    Some('*') => file_content.push_str(
                        format!(
                            "{:4}{}: '{}'",
                            "",
                            key,
                            get_reference_value(value, all_constants).unwrap().as_str().unwrap()
                        )
                        .as_str(),
                    ),
                    _ => file_content.push_str(format!("{:4}{}: '{}'", "", key, value).as_str()),
                },
                (Value::String(key), Value::Number(value)) => {
                    file_content.push_str(format!("{:4}{}: {}", "", key, value).as_str())
                }
                x => return Err(Error::NotSupportedValue(format!("{:?} and {:?}", x.0, x.1))),
            }

            if index == constant.len() - 1 {
                file_content.push_str("\n");
            } else {
                file_content.push_str(",\n");
            }
        }
        file_content.push_str("}\n");
        file.write_all(file_content.as_bytes())?;
    }

    Ok(())
}

fn get_reference_value(reference: &str, all_constants: &ConstantList) -> Option<Value> {
    let mut keys: Vec<&str> = reference.split('.').collect();
    keys[0] = keys[0].trim_start_matches('*');

    if let Some(category) = all_constants.get(keys[0]) {
        if let Some(value) = category.get(&Value::String(keys[1].to_string())) {
            Some(value.clone())
        } else {
            None
        }
    } else {
        None
    }
}
