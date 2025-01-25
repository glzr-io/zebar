#![feature(slice_internals)]

pub mod encoding;
pub mod error;
pub mod options;
pub mod process;

pub use encoding::*;
pub use error::*;
pub use options::*;
pub use process::*;

pub type Result<T> = std::result::Result<T, Error>;
