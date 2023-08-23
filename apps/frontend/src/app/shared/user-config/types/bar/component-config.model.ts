import { z } from 'zod';

import { BatteryComponentConfigSchema } from './components/battery-component-config.model';
import { ClockComponentConfigSchema } from './components/clock-component-config.model';
import { CpuComponentConfigSchema } from './components/cpu-component-config.model';
import { CustomComponentConfigSchema } from './components/custom-component-config.model';
import { GlazeWMComponentConfigSchema } from './components/glazewm-component-config.model';
import { MemoryComponentConfigSchema } from './components/memory-component-config.model';
import { NetworkComponentConfigSchema } from './components/network-component-config.model';
import { SystemTrayComponentConfigSchema } from './components/system-tray-component-config.model';
import { WeatherComponentConfigSchema } from './components/weather-component-config.model';
import { WindowTitleComponentConfigSchema } from './components/window-title-component-config.model';
import { Prettify } from '~/shared/utils';

export const ComponentConfigSchema = z.union([
  BatteryComponentConfigSchema,
  ClockComponentConfigSchema,
  CpuComponentConfigSchema,
  CustomComponentConfigSchema,
  GlazeWMComponentConfigSchema,
  MemoryComponentConfigSchema,
  NetworkComponentConfigSchema,
  SystemTrayComponentConfigSchema,
  WeatherComponentConfigSchema,
  WindowTitleComponentConfigSchema,
]);

export type ComponentConfig = Prettify<z.infer<typeof ComponentConfigSchema>>;
