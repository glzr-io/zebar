import type { Owner } from 'solid-js';

import { createBatteryProvider } from './battery/create-battery-provider';
import { createCpuProvider } from './cpu/create-cpu-provider';
import { createDateProvider } from './date/create-date-provider';
import { createGlazeWmProvider } from './glazewm/create-glazewm-provider';
import { createHostProvider } from './host/create-host-provider';
import { createIpProvider } from './ip/create-ip-provider';
import { createKomorebiProvider } from './komorebi/create-komorebi-provider';
import { createMemoryProvider } from './memory/create-memory-provider';
import { createMonitorsProvider } from './monitors/create-monitors-provider';
import { createNetworkProvider } from './network/create-network-provider';
import { createUtilProvider } from './util/create-util-provider';
import { createWeatherProvider } from './weather/create-weather-provider';
import type { ProviderConfig } from './provider-config.model';
import { ProviderType } from './provider-type.model';

export async function createProvider(
  config: ProviderConfig,
  owner: Owner,
) {
  switch (config.type) {
    case ProviderType.BATTERY:
      return createBatteryProvider(config, owner);
    case ProviderType.CPU:
      return createCpuProvider(config, owner);
    case ProviderType.DATE:
      return createDateProvider(config, owner);
    case ProviderType.GLAZEWM:
      return createGlazeWmProvider(config, owner);
    case ProviderType.HOST:
      return createHostProvider(config, owner);
    case ProviderType.IP:
      return createIpProvider(config, owner);
    case ProviderType.KOMOREBI:
      return createKomorebiProvider(config, owner);
    case ProviderType.MEMORY:
      return createMemoryProvider(config, owner);
    case ProviderType.MONITORS:
      return createMonitorsProvider(config, owner);
    case ProviderType.NETWORK:
      return createNetworkProvider(config, owner);
    case ProviderType.UTIL:
      return createUtilProvider(config, owner);
    case ProviderType.WEATHER:
      return createWeatherProvider(config, owner);
    default:
      throw new Error('Not a supported provider type.');
  }
}
