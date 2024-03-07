import { z } from 'zod';

import type { Prettify } from '~/utils';
import {
  BatteryProviderConfigSchema,
  CpuProviderConfigSchema,
  DateProviderConfigSchema,
  GlazewmProviderConfigSchema,
  HostProviderConfigSchema,
  IpProviderConfigSchema,
  KomorebiProviderConfigSchema,
  MemoryProviderConfigSchema,
  MonitorsProviderConfigSchema,
  NetworkProviderConfigSchema,
  SelfProviderConfigSchema,
  WeatherProviderConfigSchema,
} from './providers';

export const ProviderConfigSchema = z.union([
  BatteryProviderConfigSchema,
  CpuProviderConfigSchema,
  DateProviderConfigSchema,
  GlazewmProviderConfigSchema,
  HostProviderConfigSchema,
  IpProviderConfigSchema,
  KomorebiProviderConfigSchema,
  MemoryProviderConfigSchema,
  MonitorsProviderConfigSchema,
  NetworkProviderConfigSchema,
  SelfProviderConfigSchema,
  WeatherProviderConfigSchema,
]);

export type ProviderConfig = Prettify<
  z.infer<typeof ProviderConfigSchema>
>;
