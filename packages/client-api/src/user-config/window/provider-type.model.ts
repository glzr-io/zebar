import { z } from 'zod';

export enum ProviderType {
  ACTIVE_WINDOW = 'active_window',
  BATTERY = 'battery',
  CPU = 'cpu',
  DATE = 'date',
  GLAZEWM = 'glazewm',
  HOST = 'host',
  IP = 'ip',
  MEMORY = 'memory',
  MONITORS = 'monitors',
  NETWORK = 'network',
  SELF = 'self',
  SYSTEM_TRAY = 'system_tray',
  WEATHER = 'weather',
}

export const ProviderTypeSchema = z.nativeEnum(ProviderType);
