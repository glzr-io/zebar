import type { ZebarContext } from '~/zebar-context.model';
import {
  createBatteryProvider,
  type BatteryProvider,
  type BatteryProviderConfig,
} from './battery/create-battery-provider';
import {
  createCpuProvider,
  type CpuProvider,
  type CpuProviderConfig,
} from './cpu/create-cpu-provider';
import {
  createDateProvider,
  type DateProvider,
  type DateProviderConfig,
} from './date/create-date-provider';
import {
  createGlazeWmProvider,
  type GlazeWmProvider,
  type GlazeWmProviderConfig,
} from './glazewm/create-glazewm-provider';
import {
  createHostProvider,
  type HostProvider,
  type HostProviderConfig,
} from './host/create-host-provider';
import {
  createIpProvider,
  type IpProvider,
  type IpProviderConfig,
} from './ip/create-ip-provider';
import {
  createKomorebiProvider,
  type KomorebiProvider,
  type KomorebiProviderConfig,
} from './komorebi/create-komorebi-provider';
import {
  createMemoryProvider,
  type MemoryProvider,
  type MemoryProviderConfig,
} from './memory/create-memory-provider';
import {
  createNetworkProvider,
  type NetworkProvider,
  type NetworkProviderConfig,
} from './network/create-network-provider';
import {
  createWeatherProvider,
  type WeatherProvider,
  type WeatherProviderConfig,
} from './weather/create-weather-provider';
import {
  createKeyboardProvider,
  type KeyboardProvider,
  type KeyboardProviderConfig,
} from './keyboard/create-keyboard-provider';

export interface ProviderConfigMap {
  battery: BatteryProviderConfig;
  cpu: CpuProviderConfig;
  date: DateProviderConfig;
  glazewm: GlazeWmProviderConfig;
  host: HostProviderConfig;
  ip: IpProviderConfig;
  komorebi: KomorebiProviderConfig;
  memory: MemoryProviderConfig;
  network: NetworkProviderConfig;
  weather: WeatherProviderConfig;
  keyboard: KeyboardProviderConfig;
}

export interface ProviderMap {
  battery: BatteryProvider;
  cpu: CpuProvider;
  date: DateProvider;
  glazewm: GlazeWmProvider;
  host: HostProvider;
  ip: IpProvider;
  komorebi: KomorebiProvider;
  memory: MemoryProvider;
  network: NetworkProvider;
  weather: WeatherProvider;
  keyboard: KeyboardProvider;
}

export type ProviderType = keyof ProviderConfigMap;

export type ProviderConfig = ProviderConfigMap[keyof ProviderConfigMap];

export type ProviderOutput = ProviderMap[keyof ProviderMap]['output'];

/**
 * Docs {@link ZebarContext.createProvider}
 */
export function createProvider<T extends ProviderConfig>(
  config: T,
): Promise<ProviderMap[T['type']]> {
  switch (config.type) {
    case 'battery':
      return createBatteryProvider(config) as any;
    case 'cpu':
      return createCpuProvider(config) as any;
    case 'date':
      return createDateProvider(config) as any;
    case 'glazewm':
      return createGlazeWmProvider(config) as any;
    case 'host':
      return createHostProvider(config) as any;
    case 'ip':
      return createIpProvider(config) as any;
    case 'komorebi':
      return createKomorebiProvider(config) as any;
    case 'memory':
      return createMemoryProvider(config) as any;
    case 'network':
      return createNetworkProvider(config) as any;
    case 'weather':
      return createWeatherProvider(config) as any;
    case 'keyboard':
      return createKeyboardProvider(config) as any;
    default:
      throw new Error('Not a supported provider type.');
  }
}
