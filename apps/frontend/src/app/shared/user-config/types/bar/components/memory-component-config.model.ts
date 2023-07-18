import { z } from 'zod';

import { ComponentConfigBaseSchema } from '../component-config-base.model';

export const MemoryComponentConfigSchema = ComponentConfigBaseSchema.extend({
  type: z.literal('memory'),
  class_name: z.string().default('memory-component'),
  refresh_interval_ms: z.number().default(5 * 1000),
});

export type MemoryComponentConfig = z.infer<typeof MemoryComponentConfigSchema>;
