use serde::Serialize;

#[cfg(windows)]
use super::{
  audio::AudioOutput, keyboard::KeyboardOutput, komorebi::KomorebiOutput,
  media::MediaOutput,
};
use super::{
  battery::BatteryOutput, cpu::CpuOutput, disk::DiskOutput,
  host::HostOutput, ip::IpOutput, memory::MemoryOutput,
  network::NetworkOutput, weather::WeatherOutput,
};

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ProviderOutput {
  #[cfg(windows)]
  Audio(AudioOutput),
  Battery(BatteryOutput),
  Cpu(CpuOutput),
  Host(HostOutput),
  Ip(IpOutput),
  #[cfg(windows)]
  Komorebi(KomorebiOutput),
  #[cfg(windows)]
  Media(MediaOutput),
  Memory(MemoryOutput),
  Disk(DiskOutput),
  Network(NetworkOutput),
  Weather(WeatherOutput),
  #[cfg(windows)]
  Keyboard(KeyboardOutput),
}

macro_rules! impl_provider_conversions {
    // Single pattern that handles both regular and Windows variants
    ($enum_name:ident {
        // Regular variants
        $($variant:ident($type:ty)),* $(,)?

        // Optional Windows variants
        $(#[cfg(windows)] $win_variant:ident($win_type:ty)),* $(,)?
    }) => {
        // Regular implementations
        $(
            impl From<$type> for $enum_name {
                fn from(value: $type) -> Self {
                    Self::$variant(value)
                }
            }
        )*

        // Windows implementations
        $(
            #[cfg(windows)]
            impl From<$win_type> for $enum_name {
                fn from(value: $win_type) -> Self {
                    Self::$win_variant(value)
                }
            }
        )*
    };
}

// Usage is now simpler and mirrors the enum definition more closely
impl_provider_conversions!(ProviderOutput {
    Battery(BatteryOutput),
    Cpu(CpuOutput),
    Host(HostOutput),
    Ip(IpOutput),
    Memory(MemoryOutput),
    Disk(DiskOutput),
    Network(NetworkOutput),
    Weather(WeatherOutput),
    #[cfg(windows)] Komorebi(KomorebiOutput),
    #[cfg(windows)] Media(MediaOutput),
    #[cfg(windows)] Keyboard(KeyboardOutput)
});

// impl From<BatteryOutput> for ProviderOutput {
//   fn from(output: BatteryOutput) -> Self {
//     ProviderOutput::Battery(output)
//   }
// }

// impl From<CpuOutput> for ProviderOutput {
//   fn from(output: CpuOutput) -> Self {
//     ProviderOutput::Cpu(output)
//   }
// }

// impl From<HostOutput> for ProviderOutput {
//   fn from(output: HostOutput) -> Self {
//     ProviderOutput::Host(output)
//   }
// }

// impl From<IpOutput> for ProviderOutput {
//   fn from(output: IpOutput) -> Self {
//     ProviderOutput::Ip(output)
//   }
// }

// impl From<MemoryOutput> for ProviderOutput {
//   fn from(output: MemoryOutput) -> Self {
//     ProviderOutput::Memory(output)
//   }
// }

// impl From<DiskOutput> for ProviderOutput {
//   fn from(output: DiskOutput) -> Self {
//     ProviderOutput::Disk(output)
//   }
// }

// impl From<NetworkOutput> for ProviderOutput {
//   fn from(output: NetworkOutput) -> Self {
//     ProviderOutput::Network(output)
//   }
// }

// impl From<WeatherOutput> for ProviderOutput {
//   fn from(output: WeatherOutput) -> Self {
//     ProviderOutput::Weather(output)
//   }
// }

// #[cfg(windows)]
// impl From<KomorebiOutput> for ProviderOutput {
//   fn from(output: KomorebiOutput) -> Self {
//     ProviderOutput::Komorebi(output)
//   }
// }

// #[cfg(windows)]
// impl From<MediaOutput> for ProviderOutput {
//   fn from(output: MediaOutput) -> Self {
//     ProviderOutput::Media(output)
//   }
// }

// #[cfg(windows)]
// impl From<KeyboardOutput> for ProviderOutput {
//   fn from(output: KeyboardOutput) -> Self {
//     ProviderOutput::Keyboard(output)
//   }
// }
