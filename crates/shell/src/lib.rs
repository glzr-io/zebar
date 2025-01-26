#![feature(slice_internals)]

mod encoding;
mod error;
mod options;
mod shell;
mod stdout_reader;

pub use encoding::*;
pub use error::*;
pub use options::*;
pub use shell::*;
pub(crate) use stdout_reader::*;

pub type Result<T> = std::result::Result<T, Error>;
