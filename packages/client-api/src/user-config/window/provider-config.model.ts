import { z } from 'zod';

import { Prettify } from '~/utils';
import {
  ActiveWindowProviderConfigSchema,
  BatteryProviderConfigSchema,
  CpuProviderConfigSchema,
  DateProviderConfigSchema,
  GlazewmProviderConfigSchema,
  HostProviderConfigSchema,
  IpProviderConfigSchema,
  MemoryProviderConfigSchema,
  NetworkProviderConfigSchema,
  SystemTrayProviderConfigSchema,
  WeatherProviderConfigSchema,
} from './providers';

export const ProviderConfigSchema = z.union([
  ActiveWindowProviderConfigSchema,
  BatteryProviderConfigSchema,
  CpuProviderConfigSchema,
  DateProviderConfigSchema,
  GlazewmProviderConfigSchema,
  HostProviderConfigSchema,
  IpProviderConfigSchema,
  MemoryProviderConfigSchema,
  NetworkProviderConfigSchema,
  SystemTrayProviderConfigSchema,
  WeatherProviderConfigSchema,
]);

export type ProviderConfig = Prettify<z.infer<typeof ProviderConfigSchema>>;
