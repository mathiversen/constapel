use serde_json::Value;
use std::path::PathBuf;
use std::fs::{ File, OpenOptions };
use std::io::prelude::*;
use crate::{ STR_DONT_EDIT, Result };

pub fn write_to_file(value: &Value, path: &PathBuf, is_last: bool) -> Result<()> {
    let file_path = format!("{}root.css", &path.to_string_lossy());
    let file_exist = File::open(&file_path).is_ok();
    let mut file = if file_exist {
        OpenOptions::new()
            .write(true)
            .append(true)
            .open(&file_path)?
    } else {
        File::create(&file_path)?
    };
    if !file_exist {
        file.write_all(format!("/* {} */\n\n", STR_DONT_EDIT).as_bytes())?;
        file.write_all(":root {\n".as_bytes())?;
    }
    for v in value.as_object().expect("Failed to parse value") {
        let x = match v.1 {
            Value::String(x) => format!("    --{}: {};\n", v.0, x.replace("\"", "")),
            Value::Number(x) => format!("    --{}: {};\n", v.0, x),
            _ => panic!("Only strings and numbers have been implemented for"),
        };
        file.write_all(x.as_bytes()).expect("failed to write");
    }
    if is_last {
        file.write_all("}\n".as_bytes())?;
    }
    Ok(())
}