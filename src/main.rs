use colored::*;
use std::process::exit;
use std::path::PathBuf;
use structopt::StructOpt;

use constapel::Constapel;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// The .yaml input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
}

pub fn main() {
    let opt = Opt::from_args();
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
