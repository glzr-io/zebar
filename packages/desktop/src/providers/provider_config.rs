use serde::Deserialize;

use super::{
  audio::AudioProviderConfig, battery::BatteryProviderConfig,
  cpu::CpuProviderConfig, disk::DiskProviderConfig,
  host::HostProviderConfig, ip::IpProviderConfig,
  memory::MemoryProviderConfig, network::NetworkProviderConfig,
  weather::WeatherProviderConfig,
};
#[cfg(windows)]
use super::{
  keyboard::KeyboardProviderConfig, komorebi::KomorebiProviderConfig,
  media::MediaProviderConfig,
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
  #[cfg(windows)]
  Komorebi(KomorebiProviderConfig),
  #[cfg(windows)]
  Media(MediaProviderConfig),
  Memory(MemoryProviderConfig),
  Disk(DiskProviderConfig),
  Network(NetworkProviderConfig),
  Weather(WeatherProviderConfig),
  #[cfg(windows)]
  Keyboard(KeyboardProviderConfig),
}
