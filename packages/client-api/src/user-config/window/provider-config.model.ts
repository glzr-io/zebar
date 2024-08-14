import { z } from 'zod';

import type { Prettify } from '~/utils';
import {
  BatteryProviderConfigSchema,
  CpuProviderConfigSchema,
  CustomProviderConfigSchema,
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
} from './providers';

export const ProviderConfigSchema = z.union([
  BatteryProviderConfigSchema,
  CpuProviderConfigSchema,
  CustomProviderConfigSchema,
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
]);

export type ProviderConfig = Prettify<
  z.infer<typeof ProviderConfigSchema>
>;
