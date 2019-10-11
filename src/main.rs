use std::process::exit;
use colored::*;
use structopt::StructOpt;

mod cli;
mod file_types {
    // pub mod css;
    pub mod scss;
    pub mod js;
}
mod file_creator;
mod result;

use cli::Cli;
use file_creator::FileCreator;

const STR_DONT_EDIT: &str = r"DON'T EDIT THIS FILE - IT'S GENERATED";

pub fn main() {
    let opt = Cli::from_args();
    FileCreator::from_yaml(&opt.input)
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
