import { createEffect, createMemo } from 'solid-js';

import { ProviderConfig, ProviderType } from '../user-config';
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
import { log } from 'console';
import { E } from '@tauri-apps/api/path-c062430b';

export const useProviders = (
  configOrTypes: (ProviderType | ProviderConfig)[],
) => {
  const configs = createMemo(() => configOrTypes.map(toProviderConfig));

  // const providers = createMemo(() =>
  //   configOrTypes.map(configOrType => {
  //     const config =
  //       typeof configOrType === 'string'
  //         ? ({ type: configOrType } as ProviderConfig)
  //         : configOrType;

  //     return {
  //       type: config.type,
  //       ...useProvider(config),
  //     };
  //   }),
  // );

  // TODO: Need to create a namespaced map of variables. not an array.
  const variables = createMemo(() =>
    configs().reduce((acc, config) => {
      // acc[e.type] = useProvider(e).variables;
      // console.log('xxx1', useProvider(e).variables);
      // console.log('xxx2', useProvider(e));
      // console.log('acc', acc);
      // console.log('acc.hours', acc.hours);
      const variables = useProvider(config).variables;
      const clone = Object.defineProperties(
        acc[config.type],
        Object.getOwnPropertyDescriptors(variables),
      );
      return clone;
    }, {} as any),
  );
  // const variables = createMemo(() => configs().map(e => ({ acc[e.type]= e.variables })));
  const commands = createMemo(() =>
    configs().reduce((acc, e) => {
      acc[e.type] = useProvider(e).commands;
      return acc;
    }, {} as any),
  );

  function toProviderConfig(configOrType: ProviderType | ProviderConfig) {
    return typeof configOrType === 'string'
      ? ({ type: configOrType } as ProviderConfig)
      : configOrType;
  }

  function useProvider(config: ProviderConfig) {
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
        throw new Error(`Not a supported provided type '${config.type}'.`);
    }
  }

  createEffect(() => {
    // console.log('providers', providers());
    console.log('variables', variables());
  });

  return {
    variables,
    commands,
  };
};
