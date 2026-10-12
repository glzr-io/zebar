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
use crate::providers::self_managed_provider::ProviderOutput as SMProviderOutput;

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
  SelfManaged(SMProviderOutput),
}

impl_provider_output! {
  Battery(BatteryOutput),
  Cpu(CpuOutput),
  Host(HostOutput),
  Ip(IpOutput),
  Memory(MemoryOutput),
  Disk(DiskOutput),
  Network(NetworkOutput),
  Weather(WeatherOutput),
  SelfManaged(SMProviderOutput),
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
