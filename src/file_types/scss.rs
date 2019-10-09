// use serde_json::Value;
// use std::path::PathBuf;
// use std::fs::{ File };
// use std::io::prelude::*;
// use crate::{ STR_DONT_EDIT, Result };

// pub fn write_to_file(name: &str, value: &Value, path: &PathBuf) -> Result<()> {
//     let mut file_css = File::create(format!("{}/_{}.scss", &path.to_string_lossy(), &name))?;
//     file_css.write_all(format!("// {}\n\n", STR_DONT_EDIT).as_bytes())?;
//     for v in value.as_object().expect("Failed to parse value") {
//         let x = match v.1 {
//             Value::String(x) => format!("${}: {};\n", v.0, x.replace("\"", "")),
//             Value::Number(x) => format!("${}: {};\n", v.0, x),
//             _ => panic!("Only strings and numbers have been implemented for"),
//         };
//         file_css.write_all(x.as_bytes())?;
//     }
//     Ok(())
// }
use crate::{Constants, STR_DONT_EDIT, Result};

pub fn create(dir: &str, constants: &Constants) -> Result<()>{
    // dbg!(dir, constants);
    Ok(())
}