import { z } from 'zod';

import { ClockComponentConfigSchema } from './components/clock-component-config.model';
import { CpuComponentConfigSchema } from './components/cpu-component-config.model';
import { GlazeWMComponentConfigSchema } from './components/glazewm-component-config.model';
import { WeatherComponentConfigSchema } from './components/weather-component-config.model';
import { addDelimitedKey } from '../shared/add-delimited-key';
import { Prettify } from '~/shared/utils';

export const ComponentConfigSchema = z
  .discriminatedUnion('type', [
    ClockComponentConfigSchema,
    CpuComponentConfigSchema,
    GlazeWMComponentConfigSchema,
    WeatherComponentConfigSchema,
  ])
  .superRefine(addDelimitedKey('slot', z.string()));

export type ComponentConfig = Prettify<z.infer<typeof ComponentConfigSchema>>;
