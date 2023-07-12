import { z } from 'zod';

import { ElementSchema } from '../../shared/element.model';

export const CpuComponentConfigSchema = ElementSchema.extend({
  type: z.literal('cpu'),
  class_name: z.string().default('cpu-component'),
  refresh_interval_ms: z.number().default(5000),
});

export type CpuComponentConfig = z.infer<typeof CpuComponentConfigSchema>;
