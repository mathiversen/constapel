//! Constapel is a simple library to manage and create constants throughout a web application.

mod file_creator;
mod file_types;
mod result;

pub use file_creator::FileCreator as Constapel;
pub use result::{Error as ConstError, Result as ConstResult};