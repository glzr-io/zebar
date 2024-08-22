import type { BatteryProviderConfig } from './battery/create-battery-provider';
import type { CpuProviderConfig } from './cpu/create-cpu-provider';
import type { DateProviderConfig } from './date/create-date-provider';
import type { GlazeWmProviderConfig } from './glazewm/create-glazewm-provider';
import type { HostProviderConfig } from './host/create-host-provider';
import type { IpProviderConfig } from './ip/create-ip-provider';
import type { KomorebiProviderConfig } from './komorebi/create-komorebi-provider';
import type { MemoryProviderConfig } from './memory/create-memory-provider';
import type { NetworkProviderConfig } from './network/create-network-provider';
import type { UtilProviderConfig } from './util/create-util-provider';
import type { WeatherProviderConfig } from './weather/create-weather-provider';

export type ProviderConfig =
  | BatteryProviderConfig
  | CpuProviderConfig
  | DateProviderConfig
  | GlazeWmProviderConfig
  | HostProviderConfig
  | IpProviderConfig
  | KomorebiProviderConfig
  | MemoryProviderConfig
  | NetworkProviderConfig
  | UtilProviderConfig
  | WeatherProviderConfig;
