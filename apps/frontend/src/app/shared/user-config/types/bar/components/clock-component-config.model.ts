import { z } from 'zod';

import { ComponentConfigBaseSchema } from '../component-config-base.model';

export const ClockComponentConfigSchema = ComponentConfigBaseSchema.extend({
  type: z.literal('clock'),
});

export type ClockComponentConfig = z.infer<typeof ClockComponentConfigSchema>;
