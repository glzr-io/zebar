import { z } from 'zod';

import type { Prettify } from '~/utils';
import {
  BatteryProviderConfigSchema,
  CpuProviderConfigSchema,
  DateProviderConfigSchema,
  GlazeWmProviderConfigSchema,
  HostProviderConfigSchema,
  IpProviderConfigSchema,
  KomorebiProviderConfigSchema,
  MemoryProviderConfigSchema,
  MonitorsProviderConfigSchema,
  NetworkProviderConfigSchema,
  SelfProviderConfigSchema,
  UtilProviderConfigSchema,
  WeatherProviderConfigSchema,
  LanguageProviderConfigSchema,
} from './providers';

export const ProviderConfigSchema = z.union([
  BatteryProviderConfigSchema,
  CpuProviderConfigSchema,
  DateProviderConfigSchema,
  GlazeWmProviderConfigSchema,
  HostProviderConfigSchema,
  IpProviderConfigSchema,
  KomorebiProviderConfigSchema,
  MemoryProviderConfigSchema,
  MonitorsProviderConfigSchema,
  NetworkProviderConfigSchema,
  SelfProviderConfigSchema,
  UtilProviderConfigSchema,
  WeatherProviderConfigSchema,
  LanguageProviderConfigSchema,
]);

export type ProviderConfig = Prettify<
  z.infer<typeof ProviderConfigSchema>
>;
