use std::fs::{ File };
use std::io::prelude::*;
use structopt::StructOpt;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_yaml::Value;

mod cli;
use cli::{
    Cli
};

type Result<T> = std::result::Result<T, std::boxed::Box<dyn std::error::Error>>;

// const STR_DONT_EDIT: &str = r"DON'T EDIT THIS FILE - IT'S GENERATED";

type Constants = HashMap<String, HashMap<Value, Value>>;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct Inputfile {
    output_files: HashMap<String, Outputfile>,
    constants: Constants,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct Outputfile {
    path: String,
    constants: Vec<String>,
}

fn main() -> Result<()> {
    let opt = Cli::from_args();

    let mut file = File::open(&opt.input)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let serialized: Inputfile = serde_yaml::from_str(&contents).unwrap();

    for (file_ending, output_file) in serialized.output_files.iter() {
        match file_ending.as_str() {
            "js" => create_js_file(output_file, &serialized.constants),
            "scss" => create_scss_file(output_file, &serialized.constants),
            _ => unimplemented!(),
        }
    }

    Ok(())
}

fn create_js_file(output_file: &Outputfile, constants: &Constants) {
    for constant_group in output_file.constants.iter() {
        // Create file format according to the file type
        dbg!(constant_group, &constants[constant_group]);
    }
}
fn create_scss_file(output_file: &Outputfile, constants: &Constants) {
    for constant_group in output_file.constants.iter() {
        // Create file format according to the file type
        dbg!(constant_group, &constants[constant_group]);
    }
}
