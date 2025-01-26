#![feature(slice_internals)]

mod encoding;
mod error;
mod options;
mod shell;

pub use encoding::*;
pub use error::*;
pub use options::*;
pub use shell::*;

pub type Result<T> = std::result::Result<T, Error>;
