mod error;
mod tray;
mod util;

pub use error::*;
pub use tray::*;
pub use util::*;

pub type Result<T> = std::result::Result<T, Error>;
