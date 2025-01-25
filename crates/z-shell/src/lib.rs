#![feature(slice_internals)]

pub mod encoding;
pub mod error;
pub mod options;
pub mod process;

pub use error::Error;
type Result<T> = std::result::Result<T, Error>;
