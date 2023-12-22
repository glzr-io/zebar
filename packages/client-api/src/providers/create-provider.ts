import { Owner } from 'solid-js';

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
import { ProviderConfig } from '~/user-config';
import { ElementContext } from '~/element-context.model';

export async function createProvider(
  elementContext: Omit<ElementContext, 'parsedConfig' | 'providers'>,
  config: ProviderConfig,
  owner: Owner,
) {
  switch (config.type) {
    case 'active_window':
      return createActiveWindowProvider(config);
    case 'battery':
      return createBatteryProvider(config, owner);
    case 'cpu':
      return createCpuProvider(config, owner);
    case 'date':
      return createDateProvider(config, owner);
    case 'glazewm':
      return createGlazewmProvider(config, owner);
    case 'host':
      return createHostProvider(config, owner);
    case 'ip':
      return createIpProvider(config, owner);
    case 'memory':
      return createMemoryProvider(config, owner);
    case 'monitors':
      return createMonitorsProvider(config, owner);
    case 'network':
      return createNetworkProvider(config, owner);
    case 'self':
      return createSelfProvider(elementContext);
    case 'system_tray':
      return createSystemTrayProvider(config);
    case 'weather':
      return createWeatherProvider(config, owner);
    default:
      throw new Error('Not a supported provider type.');
  }
}
