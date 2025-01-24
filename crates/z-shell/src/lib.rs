#![feature(slice_internals)]

mod commands;
pub mod error;
mod process;
pub mod shell;

pub use error::Error;
type Result<T> = std::result::Result<T, Error>;
