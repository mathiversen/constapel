use derive_more::{From, Display};
use structopt::StructOpt;

mod cli;
use cli::Cli;

mod parse;
use parse::{ConstantList, Constapel};
mod file_types {
    pub mod js;
    pub mod scss;
}

#[derive(From, Debug, Display)]
pub enum Error {
    #[display(fmt="The file ending {} is not supported", _0)]
    UnknownTarget(String),
    Io(std::io::Error),
    Yaml(serde_yaml::Error),
    #[doc(hidden)]
    __Nonexhaustive,
}

pub type Result<T> = std::result::Result<T, Error>;

const STR_DONT_EDIT: &str = r"DON'T EDIT THIS FILE - IT'S GENERATED";

pub fn main() -> Result<()> {
    let opt = Cli::from_args();
    Constapel::from_yaml(&opt.input)?.run()?;
    Ok(())
}
