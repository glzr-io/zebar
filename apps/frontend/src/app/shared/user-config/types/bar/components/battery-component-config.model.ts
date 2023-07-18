import { z } from 'zod';

import { ComponentConfigBaseSchema } from '../component-config-base.model';

export const BatteryComponentConfigSchema = ComponentConfigBaseSchema.extend({
  type: z.literal('battery'),
  class_name: z.string().default('battery-component'),
  refresh_interval_ms: z.number().default(60 * 1000),
});

export type BatteryComponentConfig = z.infer<
  typeof BatteryComponentConfigSchema
>;
