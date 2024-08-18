pub mod battery;
pub mod config;
pub mod cpu;
pub mod host;
pub mod ip;
#[cfg(windows)]
pub mod komorebi;
#[cfg(all(windows, target_arch = "x86_64"))]
pub mod language;
pub mod memory;
pub mod network;
pub mod provider;
pub mod provider_manager;
pub mod provider_ref;
pub mod variables;
pub mod weather;
