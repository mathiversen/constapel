use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(From, Debug, Display)]
pub enum Error {
    #[display(fmt = "[ERROR]: The file ending .{} is not supported.", _0)]
    UnknownTarget(String),
    #[display(fmt = "[ERROR]: The value {} is of unknown type", _0)]
    NotSupportedValue(String),
    #[display(fmt = "[ERROR]: IO error.")]
    Io(std::io::Error),
    #[display(fmt = "[ERROR]: Yaml parsing error, {}", _0)]
    Yaml(serde_yaml::Error),
    #[display(fmt = "[ERROR]: The reference used could not be found, {}", _0)]
    FalsyReference(String),
    #[display(fmt = "[ERROR]: The referenced constant group doesn't exist, {}", _0)]
    NotAConstant(String),
    #[display(fmt = "[ERROR]: The config provided is unknown. {}: {}", _0, _1)]
    UnknownConfig(String, String)
}
