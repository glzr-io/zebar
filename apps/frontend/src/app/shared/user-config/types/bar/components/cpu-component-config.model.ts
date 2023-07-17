import { z } from 'zod';

import { ComponentConfigBaseSchema } from '../component-config-base.model';

export const CpuComponentConfigSchema = ComponentConfigBaseSchema.extend({
  type: z.literal('cpu'),
  class_name: z.string().default('cpu-component'),
  refresh_interval_ms: z.number().default(5 * 1000),
});

export type CpuComponentConfig = z.infer<typeof CpuComponentConfigSchema>;
