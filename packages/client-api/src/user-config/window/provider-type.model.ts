import { z } from 'zod';

export enum ProviderType {
  BATTERY = 'battery',
  CPU = 'cpu',
  DATE = 'date',
  GLAZEWM = 'glazewm',
  HOST = 'host',
  IP = 'ip',
  KOMOREBI = 'komorebi',
  MEMORY = 'memory',
  MONITORS = 'monitors',
  NETWORK = 'network',
  SELF = 'self',
  UTIL = 'util',
  WEATHER = 'weather',
  LANGUAGE = 'language',
}

export const ProviderTypeSchema = z.nativeEnum(ProviderType);
