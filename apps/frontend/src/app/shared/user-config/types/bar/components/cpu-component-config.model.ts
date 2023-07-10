import { z } from 'zod';

import { ComponentConfigBase } from '../component-config-base.model';

export const CpuComponentConfig = ComponentConfigBase.extend({
  type: z.literal('cpu'),
  refresh_interval_ms: z.number(),
});

export type CpuComponentConfig = z.infer<typeof CpuComponentConfig>;
