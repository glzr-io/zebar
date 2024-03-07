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
  NEWNETWORK = 'new_network',
  SELF = 'self',
  WEATHER = 'weather',
}

export const ProviderTypeSchema = z.nativeEnum(ProviderType);
