import { z } from 'zod';

import { ComponentConfigBaseSchema } from '../component-config-base.model';

export const ClockComponentConfigSchema = ComponentConfigBaseSchema.extend({
  type: z.literal('clock'),
  class_name: z.string().default('clock-component'),
  refresh_interval_ms: z.number().default(1000),
});

export type ClockComponentConfig = z.infer<typeof ClockComponentConfigSchema>;
