use derive_more::{From, Display};
use structopt::StructOpt;
use colored::*;

mod cli;
use cli::Cli;

mod main;
use main::{ConstantList, Constapel};
mod file_types {
    pub mod js;
    pub mod scss;
}

#[derive(From, Debug, Display)]
pub enum Error {
    #[display(fmt="[ERROR]: The file ending .{} is not supported.", _0)]
    UnknownTarget(String),
    #[display(fmt="[ERROR]: IO error.")]
    Io(std::io::Error),
    #[display(fmt="[ERROR]: Yaml parsing error")]
    Yaml(serde_yaml::Error)
}

pub type Result<T> = std::result::Result<T, Error>;

const STR_DONT_EDIT: &str = r"DON'T EDIT THIS FILE - IT'S GENERATED";

pub fn main() {
    let opt = Cli::from_args();
    Constapel::from_yaml(&opt.input)
      .map_err(|error| {
        println!("{}", error.to_string().red());
        std::process::exit(1)
      })
      .expect("Failed to unwrap result")
      .run()
      .map_err(|error| println!("{}", error.to_string().red()));
}
