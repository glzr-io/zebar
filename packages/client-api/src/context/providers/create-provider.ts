import { createActiveWindowProvider } from './active-window/create-active-window-provider';
import { createBatteryProvider } from './battery/create-battery-provider';
import { createCpuProvider } from './cpu/create-cpu-provider';
import { createDateTimeProvider } from './date-time/create-date-time-provider';
import { createGlazewmProvider } from './glazewm/create-glazewm-provider';
import { createHostProvider } from './host/create-host-provider';
import { createIpProvider } from './ip/create-ip-provider';
import { createMemoryProvider } from './memory/create-memory-provider';
import { createNetworkProvider } from './network/create-network-provider';
import { createSystemTrayProvider } from './system-tray/create-system-tray-provider';
import { createWeatherProvider } from './weather/create-weather-provider';
import { ProviderConfig } from '~/user-config';
import { memoize } from '~/utils';

export const createProvider = memoize((config: ProviderConfig) => {
  switch (config.type) {
    case 'active_window':
      return createActiveWindowProvider(config);
    case 'battery':
      return createBatteryProvider(config);
    case 'cpu':
      return createCpuProvider(config);
    case 'date_time':
      return createDateTimeProvider(config);
    case 'glazewm':
      return createGlazewmProvider(config);
    case 'host':
      return createHostProvider(config);
    case 'ip':
      return createIpProvider(config);
    case 'memory':
      return createMemoryProvider(config);
    case 'network':
      return createNetworkProvider(config);
    case 'system_tray':
      return createSystemTrayProvider(config);
    case 'weather':
      return createWeatherProvider(config);
    default:
      throw new Error('Not a supported provider type.');
  }
});
