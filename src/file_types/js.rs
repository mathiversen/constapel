// use serde_json::Value;
// use std::path::PathBuf;
// use std::fs::{ File };
// use std::io::prelude::*;
// use crate::{ STR_DONT_EDIT, Result };

// pub fn write_to_file(name: &str, value: &Value, path: &PathBuf) -> Result<()> {
//     let mut file_css = File::create(format!("{}/{}.js", &path.to_string_lossy(), &name))?;
//     file_css.write_all(format!("// {}\n\n", STR_DONT_EDIT).as_bytes())?;
//     file_css.write_all("export default {\n".as_bytes())?;
//     for v in value.as_object().expect("Failed to parse value") {
//         file_css.write_all(format!("    {}: {},\n", v.0, v.1).as_bytes())?;
//     }
//     file_css.write_all("}\n".as_bytes())?;
//     Ok(())
// }

use std::io::Write;
use std::fs;
use crate::{Constants, STR_DONT_EDIT, Result};

pub fn create(dir_path: &str, constants: &Constants) -> Result<()> {
    // Create dir
    if fs::metadata(dir_path).is_err() {
        fs::create_dir_all(dir_path).expect("Failed to create directory");
    }

    for (constant_group, constant) in constants.iter() {
        let mut file = fs::File::create(format!("{}/{}.js", dir_path, constant_group)).expect("Failed to create file.");
        file.write_all(format!("// {}\n\n", STR_DONT_EDIT).as_bytes())?;
        file.write_all("export default {\n".as_bytes())?;
        for (key, value) in constant.iter() {
            file.write_all(format!("    {:?}: {:?},\n", key, value).as_bytes())?;
        }
        file.write_all("}\n".as_bytes())?;
    }
    // Write to file

    dbg!(dir_path, constants);
    Ok(())
}