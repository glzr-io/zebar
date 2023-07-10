import { z } from 'zod';

import { ElementSchema } from '../../shared/element.model';

export const CpuComponentConfigSchema = ElementSchema.extend({
  type: z.literal('cpu'),
  refresh_interval_ms: z.number(),
});

export type CpuComponentConfig = z.infer<typeof CpuComponentConfigSchema>;
