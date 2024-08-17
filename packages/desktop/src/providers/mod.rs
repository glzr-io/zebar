pub mod battery;
pub mod config;
pub mod cpu;
pub mod host;
pub mod interval_provider;
pub mod ip;
#[cfg(all(windows, target_arch = "x86_64"))]
pub mod komorebi;
pub mod manager;
pub mod memory;
pub mod network;
pub mod provider;
pub mod provider_ref;
pub mod variables;
pub mod weather;
