import { createStore } from 'solid-js/store';

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
import {
  ProviderConfig,
  ProvidersConfigSchema,
  UserConfig,
  getBarConfigs,
  getGroupConfigs,
} from '../user-config';
import { memoize } from '../utils';
import { ProviderNode } from './provider-node.model';

export const useProviderTree = memoize(() => {
  const [value, setProviderTree] = createStore<ProviderNode>({
    id: 'root',
    variables: {},
    functions: {},
    slots: {},
    parent: null,
    children: [],
  });

  // createEffect(on(userConfig.config, config => {}));

  function update(configObj: unknown) {
    // Need to traverse every `provider` and `variables` property.
    const root: ProviderNode = {
      id: 'root',
      variables: {},
      functions: {},
      slots: {},
      parent: null,
      children: [],
    };

    for (const barconfig of getBarConfigs(configObj as UserConfig)) {
      const barProviders = ProvidersConfigSchema.parse(
        barconfig.providers ?? [],
      );
      const variables = getVariables(barProviders);
      const functions = getFunctions(barProviders);

      const barNode: ProviderNode = {
        id: 'bar',
        variables,
        functions,
        slots: {},
        parent: root,
        children: [] as ProviderNode[],
      };
      root.children.push(barNode);

      for (const groupConfig of getGroupConfigs(barconfig)) {
        const groupProviders = ProvidersConfigSchema.parse(
          groupConfig.providers ?? [],
        );
        const variables = getVariables(groupProviders);
        const functions = getFunctions(groupProviders);

        const groupNode: ProviderNode = {
          id: 'group',
          variables,
          functions,
          slots: {},
          parent: barNode,
          children: [],
        };
        barNode.children.push(groupNode);

        for (const componentConfig of groupConfig.components) {
          const componentProviders = ProvidersConfigSchema.parse(
            componentConfig.providers ?? [],
          );
          const variables = getVariables(componentProviders);
          const functions = getFunctions(componentProviders);

          const componentNode: ProviderNode = {
            id: 'component',
            variables,
            functions,
            slots: {},
            parent: groupNode,
            children: [],
          };
          groupNode.children.push(componentNode);
        }
      }
    }

    setProviderTree(root);
    return value;
  }

  function getVariables(configs: ProviderConfig[]) {
    return configs.reduce(
      (acc, config) => ({
        ...acc,
        [config.type]: useProvider(config).variables,
      }),
      {},
    );
  }

  function getFunctions(configs: ProviderConfig[]) {
    return configs.reduce(
      (acc, config) => ({
        ...acc,
        [config.type]: useProvider(config).commands,
      }),
      {},
    );
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
        throw new Error(`Not a supported provider type '${config.type}'.`);
    }
  }

  return {
    value,
    update,
  };
});
