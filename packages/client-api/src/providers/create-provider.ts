import type { Owner } from 'solid-js';

import {
  createBatteryProvider,
  type BatteryProvider,
} from './battery/create-battery-provider';
import {
  createCpuProvider,
  type CpuProvider,
} from './cpu/create-cpu-provider';
import {
  createDateProvider,
  type DateProvider,
} from './date/create-date-provider';
import {
  createGlazeWmProvider,
  type GlazeWmProvider,
} from './glazewm/create-glazewm-provider';
import {
  createHostProvider,
  type HostProvider,
} from './host/create-host-provider';
import {
  createIpProvider,
  type IpProvider,
} from './ip/create-ip-provider';
import {
  createKomorebiProvider,
  type KomorebiProvider,
} from './komorebi/create-komorebi-provider';
import {
  createMemoryProvider,
  type MemoryProvider,
} from './memory/create-memory-provider';
import {
  createNetworkProvider,
  type NetworkProvider,
} from './network/create-network-provider';
import {
  createUtilProvider,
  type UtilProvider,
} from './util/create-util-provider';
import {
  createWeatherProvider,
  type WeatherProvider,
} from './weather/create-weather-provider';
import type { ProviderConfig } from './provider-config.model';
import { ProviderType } from './provider-type.model';

// type ProviderTypeToProvider = {
//   [ProviderType.BATTERY]: BatteryProvider;
//   [ProviderType.CPU]: CpuProvider;
//   [ProviderType.DATE]: DateProvider;
//   [ProviderType.GLAZEWM]: GlazeWmProvider;
//   [ProviderType.HOST]: HostProvider;
//   [ProviderType.IP]: IpProvider;
//   [ProviderType.KOMOREBI]: KomorebiProvider;
//   [ProviderType.MEMORY]: MemoryProvider;
//   [ProviderType.NETWORK]: NetworkProvider;
//   [ProviderType.UTIL]: UtilProvider;
//   [ProviderType.WEATHER]: WeatherProvider;
// };

async function aa() {
  const xx = await createProvider({ type: ProviderType.UTIL }, {} as any);
}

// const createProviderMap = {
//   [ProviderType.BATTERY]: createBatteryProvider,
//   [ProviderType.CPU]: createCpuProvider,
//   [ProviderType.DATE]: createDateProvider,
//   [ProviderType.GLAZEWM]: createGlazeWmProvider,
//   [ProviderType.HOST]: createHostProvider,
//   [ProviderType.IP]: createIpProvider,
//   [ProviderType.KOMOREBI]: createKomorebiProvider,
//   [ProviderType.MEMORY]: createMemoryProvider,
//   [ProviderType.NETWORK]: createNetworkProvider,
//   [ProviderType.UTIL]: createUtilProvider,
//   [ProviderType.WEATHER]: createWeatherProvider,
// } as const;

// type ProviderCreator<T extends ProviderConfig, K> = (
//   config: T,
//   owner: Owner,
// ) => Promise<K>;

// type ProviderCreators = {
//   [T in ProviderType]: ProviderCreator<
//     Extract<ProviderConfig, { type: K }>
//   >;
// };

// const providerCreators: ProviderCreators = {
//   [ProviderType.BATTERY]: createBatteryProvider,
//   [ProviderType.CPU]: createCpuProvider,
//   [ProviderType.DATE]: createDateProvider,
//   [ProviderType.GLAZEWM]: createGlazeWmProvider,
//   [ProviderType.HOST]: createHostProvider,
//   [ProviderType.IP]: createIpProvider,
//   [ProviderType.KOMOREBI]: createKomorebiProvider,
//   [ProviderType.MEMORY]: createMemoryProvider,
//   [ProviderType.NETWORK]: createNetworkProvider,
//   [ProviderType.UTIL]: createUtilProvider,
//   [ProviderType.WEATHER]: createWeatherProvider,
// };

// export async function createProvider<T extends ProviderConfig>(
//   config: T,
//   owner: Owner,
// ): Promise<ReturnType<ProviderCreators[T['type']]>> {
//   const creator = providerCreators[config.type] as ProviderCreator<T>;

//   if (!creator) {
//     throw new Error('Not a supported provider type.');
//   }

//   return creator(config, owner);
// }

// ): Promise<ProviderTypeToProvider[T['type']]> {
// ) {
//   const createProviderMap = {
//     [ProviderType.BATTERY]: createBatteryProvider,
//     [ProviderType.CPU]: createCpuProvider,
//     [ProviderType.DATE]: createDateProvider,
//     [ProviderType.GLAZEWM]: createGlazeWmProvider,
//     [ProviderType.HOST]: createHostProvider,
//     [ProviderType.IP]: createIpProvider,
//     [ProviderType.KOMOREBI]: createKomorebiProvider,
//     [ProviderType.MEMORY]: createMemoryProvider,
//     [ProviderType.NETWORK]: createNetworkProvider,
//     [ProviderType.UTIL]: createUtilProvider,
//     [ProviderType.WEATHER]: createWeatherProvider,
//   } as const;

//   return createProviderMap[config.type](config as any, owner);
//   // const providerFn = createProviderMap[config.type] as (
//   //   config: any,
//   //   owner: Owner,
//   // ) => Promise<ProviderTypeToProvider[T['type']]>;

//   // if (!providerFn) {
//   //   throw new Error('Not a supported provider type.');
//   // }

//   // return providerFn(config, owner);
// }

const providerCreators = {
  [ProviderType.BATTERY]: createBatteryProvider,
  [ProviderType.CPU]: createCpuProvider,
  [ProviderType.DATE]: createDateProvider,
  [ProviderType.GLAZEWM]: createGlazeWmProvider,
  [ProviderType.HOST]: createHostProvider,
  [ProviderType.IP]: createIpProvider,
  [ProviderType.KOMOREBI]: createKomorebiProvider,
  [ProviderType.MEMORY]: createMemoryProvider,
  [ProviderType.NETWORK]: createNetworkProvider,
  [ProviderType.UTIL]: createUtilProvider,
  [ProviderType.WEATHER]: createWeatherProvider,
} as const;

type ProviderCreatorMap = typeof providerCreators;
type ProviderConfigType = ProviderConfig & {
  type: keyof ProviderCreatorMap;
};

export async function createProvider<T extends ProviderConfigType>(
  config: T,
  owner: Owner,
): Promise<ReturnType<ProviderCreatorMap[T['type']]>> {
  const creator = providerCreators[config.type];

  if (!creator) {
    throw new Error('Not a supported provider type.');
  }

  return creator(config as any, owner) as any;
}
