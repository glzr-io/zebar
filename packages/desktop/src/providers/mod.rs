#[cfg(windows)]
mod audio;
mod battery;
mod cpu;
mod disk;
mod host;
mod ip;
#[cfg(windows)]
mod keyboard;
#[cfg(any(target_os = "macos", windows))]
mod komorebi;
#[cfg(windows)]
mod media;
mod memory;
mod network;
mod provider;
mod provider_config;
mod provider_function;
mod provider_manager;
mod provider_output;
#[cfg(windows)]
mod systray;
mod weather;

#[cfg(any(
  target_os = "linux",
  target_os = "dragonfly",
  target_os = "freebsd",
  target_os = "netbsd",
  target_os = "openbsd"
))]
pub mod sway;

pub use provider::*;
pub use provider_config::*;
pub use provider_function::*;
pub use provider_manager::*;
pub use provider_output::*;
