use serde::Deserialize;

#[cfg(windows)]
use super::{
  audio::AudioProviderConfig, keyboard::KeyboardProviderConfig,
  komorebi::KomorebiProviderConfig, media::MediaProviderConfig,
};
use super::{
  battery::BatteryProviderConfig, cpu::CpuProviderConfig,
  disk::DiskProviderConfig, focused_window::FocusedWindowProviderConfig,
  host::HostProviderConfig, ip::IpProviderConfig,
  memory::MemoryProviderConfig, network::NetworkProviderConfig,
  weather::WeatherProviderConfig,
};

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ProviderConfig {
  #[cfg(windows)]
  Audio(AudioProviderConfig),
  Battery(BatteryProviderConfig),
  Cpu(CpuProviderConfig),
  FocusedWindow(FocusedWindowProviderConfig),
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
