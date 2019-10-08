use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
pub struct Cli {
    /// The .yaml input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
}