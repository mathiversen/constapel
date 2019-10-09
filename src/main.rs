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

mod file_types {
    pub mod js;
    pub mod scss;
}

type Result<T> = std::result::Result<T, std::boxed::Box<dyn std::error::Error>>;


const STR_DONT_EDIT: &str = r"DON'T EDIT THIS FILE - IT'S GENERATED";

type Constants = HashMap<String, HashMap<Value, Value>>;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Inputfile {
    output_files: HashMap<String, Outputfile>,
    constants: Constants,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Outputfile {
    path: String,
    constants: Vec<String>,
}

fn main() -> Result<()> {
    let opt = Cli::from_args();

    let mut file = File::open(&opt.input)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let serialized: Inputfile = serde_yaml::from_str(&contents).expect("Failed to parse the yaml file.");

    for (file_ending, output_file) in serialized.output_files.iter() {
        let relevant_constants: Constants = serialized.constants.iter().fold(HashMap::new(), |mut acc, (key, value)| {
            if output_file.constants.contains(&key) {
                acc.insert(key.clone(), value.clone());
                acc
            } else {
                acc
            }
        });
        match file_ending.as_str() {
            "js" => file_types::js::create(&output_file.path, &relevant_constants)?,
            "scss" => file_types::scss::create(&output_file.path, &relevant_constants)?,
            _ => unimplemented!(),
        }
    }

    Ok(())
}
