import { z } from 'zod';

import { ComponentConfigBaseSchema } from '../component-config-base.model';

export const CpuComponentConfigSchema = ComponentConfigBaseSchema.extend({
  type: z.literal('cpu'),
  refresh_interval_ms: z.number(),
});

export type CpuComponentConfig = z.infer<typeof CpuComponentConfigSchema>;
