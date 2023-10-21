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

export const useProvider = memoize((options: ProviderConfig) => {
  switch (options.type) {
    case 'active_window':
      return useActiveWindowProvider(options);
    case 'battery':
      return useBatteryProvider(options);
    case 'cpu':
      return useCpuProvider(options);
    case 'date_time':
      return useDateTimeProvider(options);
    case 'glazewm':
      return useGlazewmProvider(options);
    case 'ip':
      return useIpProvider(options);
    case 'memory':
      return useMemoryProvider(options);
    case 'network':
      return useNetworkProvider(options);
    case 'system_tray':
      return useSystemTrayProvider(options);
    case 'weather':
      return useWeatherProvider(options);
    default:
      throw new Error('Not a supported provider type.');
  }
});
