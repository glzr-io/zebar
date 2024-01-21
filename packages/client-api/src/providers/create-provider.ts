import type { Owner } from 'solid-js';

import { createActiveWindowProvider } from './active-window/create-active-window-provider';
import { createBatteryProvider } from './battery/create-battery-provider';
import { createCpuProvider } from './cpu/create-cpu-provider';
import { createDateProvider } from './date/create-date-provider';
import { createGlazewmProvider } from './glazewm/create-glazewm-provider';
import { createHostProvider } from './host/create-host-provider';
import { createIpProvider } from './ip/create-ip-provider';
import { createMemoryProvider } from './memory/create-memory-provider';
import { createMonitorsProvider } from './monitors/create-monitors-provider';
import { createNetworkProvider } from './network/create-network-provider';
import { createSelfProvider } from './self/create-self-provider';
import { createSystemTrayProvider } from './system-tray/create-system-tray-provider';
import { createWeatherProvider } from './weather/create-weather-provider';
import { ProviderType, type ProviderConfig } from '~/user-config';
import type { ElementContext } from '~/element-context.model';
import type { PickPartial } from '~/utils';

export async function createProvider(
  elementContext: PickPartial<
    ElementContext,
    'parsedConfig' | 'providers'
  >,
  config: ProviderConfig,
  owner: Owner,
) {
  switch (config.type) {
    case ProviderType.ACTIVE_WINDOW:
      return createActiveWindowProvider(config);
    case ProviderType.BATTERY:
      return createBatteryProvider(config, owner);
    case ProviderType.CPU:
      return createCpuProvider(config, owner);
    case ProviderType.DATE:
      return createDateProvider(config, owner);
    case ProviderType.GLAZEWM:
      return createGlazewmProvider(config, owner);
    case ProviderType.HOST:
      return createHostProvider(config, owner);
    case ProviderType.IP:
      return createIpProvider(config, owner);
    case ProviderType.MEMORY:
      return createMemoryProvider(config, owner);
    case ProviderType.MONITORS:
      return createMonitorsProvider(config, owner);
    case ProviderType.NETWORK:
      return createNetworkProvider(config, owner);
    case ProviderType.SELF:
      return createSelfProvider(elementContext);
    case ProviderType.SYSTEM_TRAY:
      return createSystemTrayProvider(config);
    case ProviderType.WEATHER:
      return createWeatherProvider(config, owner);
    default:
      throw new Error('Not a supported provider type.');
  }
}
