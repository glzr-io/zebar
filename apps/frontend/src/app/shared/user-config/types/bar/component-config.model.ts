import { z } from 'zod';

import { ClockComponentConfig } from './components/clock-component-config.model';
import { CpuComponentConfig } from './components/cpu-component-config.model';
import { GlazeWMComponentConfig } from './components/glazewm-component-config.model';

export const ComponentConfig = z.discriminatedUnion('type', [
  ClockComponentConfig,
  CpuComponentConfig,
  GlazeWMComponentConfig,
]);

export type ComponentConfig = z.infer<typeof ComponentConfig>;
