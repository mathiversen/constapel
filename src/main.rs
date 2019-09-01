use serde_json::Value;
use std::fs::{ File };
use std::io::prelude::*;
use structopt::StructOpt;

mod cli;
use cli::{
    Cli,
    Validator
};

mod file_types {
    pub mod css;
    pub mod scss;
    pub mod js;
}

type Result<T> = std::result::Result<T, std::boxed::Box<dyn std::error::Error>>;

const STR_DONT_EDIT: &str = r"DON'T EDIT THIS FILE - IT'S GENERATED";

fn main() -> Result<()> {
    let opt = Cli::from_args().is_valid();

    let mut file = File::open(&opt.input)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let root: Value = serde_json::from_str(&contents)?;
    let root = root.as_object().expect("The file provided is not a json");
    
    for (index, (key, value)) in root.iter().enumerate() {
        if opt.dir_scss.is_some() {
            file_types::scss::write_to_file(key, value, &opt.dir_scss.clone().unwrap())?;
        }
        if opt.dir_css.is_some() {
            let last_value = index == root.len() - 1;
            file_types::css::write_to_file(value, &opt.dir_css.clone().unwrap(), last_value)?;
        }
        if opt.dir_js.is_some() {
            file_types::js::write_to_file(key, value, &opt.dir_js.clone().unwrap())?;
        }
    }
    Ok(())
}
