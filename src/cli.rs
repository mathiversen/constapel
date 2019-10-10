use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    /// The .yaml input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
}
