mod error;
mod systray;
mod tray_spy;
mod util;
mod window;

pub use error::*;
pub use systray::*;
pub use tray_spy::*;
pub(crate) use util::*;
pub(crate) use window::*;

pub type Result<T> = std::result::Result<T, Error>;
