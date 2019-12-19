//! Constapel is a simple library to manage and create constants throughout a web application.

pub mod constapel;
pub mod prelude;

pub use crate::constapel::Constapel;
pub use crate::prelude::{Error, Result};