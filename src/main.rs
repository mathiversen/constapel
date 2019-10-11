use colored::*;
use std::process::exit;
use structopt::StructOpt;

mod cli;
mod file_types {
    // pub mod css;
    pub mod js;
    pub mod scss;
}
mod file_creator;
mod result;

use cli::Cli;
use file_creator::FileCreator as Constapel;

const STR_DONT_EDIT: &str = r"DON'T EDIT THIS FILE - IT'S GENERATED";

pub fn main() {
    let opt = Cli::from_args();
    Constapel::from_yaml_file(&opt.input)
        .map_err(|error| {
            println!("{}", error.to_string().red());
            exit(1)
        })
        .expect("Failed to unwrap result.")
        .run()
        .map_err(|error| {
            println!("{}", error.to_string().red());
            exit(1)
        })
        .expect("Failed to unwrap result.");
}
