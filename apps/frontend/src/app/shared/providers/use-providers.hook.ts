import { ProviderConfig, ProviderType } from '../user-config';

export const useProviders = (
  providerConfigs: (ProviderType | ProviderConfig)[],
) => {
  function getProvider(type: ProviderType) {
    switch (type) {
      case 'active_window':
        return useActiveWindowProvider();
      case 'battery':
        return useBatteryProvider();
      case 'cpu':
        return useCpuProvider();
      case 'date_time':
        return useDateTimeProvider();
      case 'glazewm':
        return useGlazewmProvider();
      case 'ip':
        return useIpProvider();
      case 'memory':
        return useMemoryProvider();
      case 'network':
        return useNetworkProvider();
      case 'system_tray':
        return useSystemTrayProvider();
      case 'weather':
        return useWeatherProvider();
      default:
        throw new Error(`Not a supported provided type '${type}'.`);
    }
  }

  return {
    variables: {},
    commands: {},
  };
};
