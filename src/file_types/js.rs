use serde_json::Value;
use std::path::PathBuf;
use std::fs::{ File };
use std::io::prelude::*;
use crate::{ STR_DONT_EDIT, Result };

pub fn write_to_file(name: &str, value: &Value, path: &PathBuf) -> Result<()> {
    let mut file_css = File::create(format!("{}/{}.js", &path.to_string_lossy(), &name))?;
    file_css.write_all(format!("// {}\n\n", STR_DONT_EDIT).as_bytes())?;
    file_css.write_all("export default {\n".as_bytes())?;
    for v in value.as_object().expect("Failed to parse value") {
        file_css.write_all(format!("    {}: {},\n", v.0, v.1).as_bytes())?;
    }
    file_css.write_all("}\n".as_bytes())?;
    Ok(())
}