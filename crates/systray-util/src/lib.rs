mod error;
mod tray;
mod util;
mod window;

pub use error::*;
pub use tray::*;
pub use util::*;
pub use window::*;

pub type Result<T> = std::result::Result<T, Error>;
