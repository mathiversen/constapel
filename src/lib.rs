//! Constapel is a simple library to manage and create constants throughout a web application.

mod file_creator;
mod file_types;
mod utils;

pub use file_creator::FileCreator as Constapel;
pub use utils::{Error as ConstError, Result as ConstResult};