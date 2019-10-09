use structopt::StructOpt;
use std::collections::HashMap;

mod cli;
use cli::{
    Cli
};

mod parse;
use parse::{
    Constants
};
mod file_types {
    pub mod js;
    pub mod scss;
}

type Result<T> = std::result::Result<T, std::boxed::Box<dyn std::error::Error>>;


const STR_DONT_EDIT: &str = r"DON'T EDIT THIS FILE - IT'S GENERATED";

fn main() -> Result<()> {
    let opt = Cli::from_args();

    let serialized = parse::serialize_input_file(&opt.input)?;

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
