use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(From, Debug, Display)]
pub enum Error {
    #[display(fmt = "[ERROR]: The file ending .{} is not supported.", _0)]
    UnknownTarget(String),
    #[display(fmt = "[ERROR]: The value {} is of an unknown type, only strings and numbers are supported.", _0)]
    NotSupportedValue(String),
    #[display(fmt = "[ERROR]: IO error.")]
    Io(std::io::Error),
    #[display(fmt = "[ERROR]: Yaml parsing error")]
    Yaml(serde_yaml::Error),
}