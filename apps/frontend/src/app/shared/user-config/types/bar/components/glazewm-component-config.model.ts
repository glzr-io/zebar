import { z } from 'zod';

import { ComponentConfigBaseSchema } from '../component-config-base.model';

export const GlazeWMComponentConfigSchema = ComponentConfigBaseSchema.extend({
  type: z.literal('glazewm'),
});

export type GlazeWMComponentConfig = z.infer<
  typeof GlazeWMComponentConfigSchema
>;
