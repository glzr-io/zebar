use serde::Deserialize;

#[cfg(any(target_os = "macos", windows))]
use super::komorebi::KomorebiProviderConfig;
#[cfg(windows)]
use super::{
  audio::AudioProviderConfig, keyboard::KeyboardProviderConfig,
  media::MediaProviderConfig, systray::SystrayProviderConfig,
};
use super::{
  battery::BatteryProviderConfig, cpu::CpuProviderConfig,
  disk::DiskProviderConfig, host::HostProviderConfig,
  ip::IpProviderConfig, memory::MemoryProviderConfig,
  network::NetworkProviderConfig, weather::WeatherProviderConfig,
};

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProviderConfig {
  #[cfg(windows)]
  Audio(AudioProviderConfig),
  Battery(BatteryProviderConfig),
  Cpu(CpuProviderConfig),
  Host(HostProviderConfig),
  Ip(IpProviderConfig),
  #[cfg(any(target_os = "macos", windows))]
  Komorebi(KomorebiProviderConfig),
  #[cfg(windows)]
  Media(MediaProviderConfig),
  Memory(MemoryProviderConfig),
  Disk(DiskProviderConfig),
  Network(NetworkProviderConfig),
  #[cfg(windows)]
  Systray(SystrayProviderConfig),
  Weather(WeatherProviderConfig),
  #[cfg(windows)]
  Keyboard(KeyboardProviderConfig),
}
