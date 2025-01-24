#![feature(slice_internals)]

pub mod commands;
pub mod error;
pub mod process;
pub mod shell;

pub use error::Error;
type Result<T> = std::result::Result<T, Error>;
