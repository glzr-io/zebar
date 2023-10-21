import { useActiveWindowProvider } from './active-window/use-active-window-provider.hook';
import { useBatteryProvider } from './battery/use-battery-provider.hook';
import { useCpuProvider } from './cpu/use-cpu-provider.hook';
import { useDateTimeProvider } from './date-time/use-date-time-provider.hook';
import { useGlazewmProvider } from './glazewm/use-glazewm-provider.hook';
import { useIpProvider } from './ip/use-ip-provider.hook';
import { useMemoryProvider } from './memory/use-memory-provider.hook';
import { useNetworkProvider } from './network/use-network-provider.hook';
import { useSystemTrayProvider } from './system-tray/use-system-tray-provider.hook';
import { useWeatherProvider } from './weather/use-weather-provider.hook';
import { ProviderConfig } from '../user-config';
import { memoize } from '../utils';

export const useProvider = memoize((config: ProviderConfig) => {
  switch (config.type) {
    case 'active_window':
      return useActiveWindowProvider(config);
    case 'battery':
      return useBatteryProvider(config);
    case 'cpu':
      return useCpuProvider(config);
    case 'date_time':
      return useDateTimeProvider(config);
    case 'glazewm':
      return useGlazewmProvider(config);
    case 'ip':
      return useIpProvider(config);
    case 'memory':
      return useMemoryProvider(config);
    case 'network':
      return useNetworkProvider(config);
    case 'system_tray':
      return useSystemTrayProvider(config);
    case 'weather':
      return useWeatherProvider(config);
    default:
      throw new Error('Not a supported provider type.');
  }
});
