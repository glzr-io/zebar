use serde::Serialize;

#[cfg(any(target_os = "macos", windows))]
use super::komorebi::KomorebiOutput;
#[cfg(windows)]
use super::{
  audio::AudioOutput, keyboard::KeyboardOutput, media::MediaOutput,
  systray::SystrayOutput,
};
use super::{
  battery::BatteryOutput, cpu::CpuOutput, disk::DiskOutput,
  host::HostOutput, ip::IpOutput, memory::MemoryOutput,
  network::NetworkOutput, weather::WeatherOutput,
};

#[cfg(any(
  target_os = "linux",
  target_os = "dragonfly",
  target_os = "freebsd",
  target_os = "netbsd",
  target_os = "openbsd"
))]
use crate::providers::sway::SwayOutput;


/// Implements `From<T>` for `ProviderOutput` for each given variant.
macro_rules! impl_provider_output {
  ($($variant:ident($type:ty)),* $(,)?) => {
    $(
      impl From<$type> for ProviderOutput {
        fn from(value: $type) -> Self {
          Self::$variant(value)
        }
      }
    )*
  };
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ProviderOutput {
  #[cfg(windows)]
  Audio(AudioOutput),
  Battery(BatteryOutput),
  Cpu(CpuOutput),
  Host(HostOutput),
  Ip(IpOutput),
  #[cfg(any(target_os = "macos", windows))]
  Komorebi(KomorebiOutput),
  #[cfg(windows)]
  Media(MediaOutput),
  Memory(MemoryOutput),
  Disk(DiskOutput),
  Network(NetworkOutput),
  #[cfg(windows)]
  Systray(SystrayOutput),
  Weather(WeatherOutput),
  #[cfg(windows)]
  Keyboard(KeyboardOutput),
  #[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
  ))]
  Sway(SwayOutput),
}

impl_provider_output! {
  Battery(BatteryOutput),
  Cpu(CpuOutput),
  Host(HostOutput),
  Ip(IpOutput),
  Memory(MemoryOutput),
  Disk(DiskOutput),
  Network(NetworkOutput),
  Weather(WeatherOutput)
}

#[cfg(target_os = "macos")]
impl_provider_output! {
  Komorebi(KomorebiOutput),
}

#[cfg(windows)]
impl_provider_output! {
  Audio(AudioOutput),
  Media(MediaOutput),
  Keyboard(KeyboardOutput),
  Komorebi(KomorebiOutput),
  Systray(SystrayOutput),
}

#[cfg(any(
  target_os = "linux",
  target_os = "dragonfly",
  target_os = "freebsd",
  target_os = "netbsd",
  target_os = "openbsd"
))]
impl_provider_output! {
  Sway(SwayOutput)
}

