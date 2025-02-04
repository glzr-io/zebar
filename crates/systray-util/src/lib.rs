mod error;
mod systray;
mod tray_spy;
mod util;

pub use error::*;
pub use systray::*;
pub use tray_spy::*;
pub(crate) use util::*;

pub type Result<T> = std::result::Result<T, Error>;
