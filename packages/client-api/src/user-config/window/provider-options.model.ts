import { z } from 'zod';

import { Prettify } from '~/utils';
import {
  ActiveWindowProviderOptionsSchema,
  BatteryProviderOptionsSchema,
  CpuProviderOptionsSchema,
  DateTimeProviderOptionsSchema,
  GlazewmProviderOptionsSchema,
  IpProviderOptionsSchema,
  MemoryProviderOptionsSchema,
  NetworkProviderOptionsSchema,
  SystemTrayProviderOptionsSchema,
  WeatherProviderOptionsSchema,
} from './providers';

export const ProviderOptionsSchema = z.union([
  ActiveWindowProviderOptionsSchema,
  BatteryProviderOptionsSchema,
  CpuProviderOptionsSchema,
  DateTimeProviderOptionsSchema,
  GlazewmProviderOptionsSchema,
  IpProviderOptionsSchema,
  MemoryProviderOptionsSchema,
  NetworkProviderOptionsSchema,
  SystemTrayProviderOptionsSchema,
  WeatherProviderOptionsSchema,
]);

export type ProviderOptions = Prettify<z.infer<typeof ProviderOptionsSchema>>;
