import { z } from 'zod';

import { ComponentConfigBaseSchema } from '../component-config-base.model';

export const GlazeWMComponentConfigSchema = ComponentConfigBaseSchema.extend({
  type: z.literal('glazewm'),
  class_name: z.string().default('glazewm-component'),
});

export type GlazeWMComponentConfig = z.infer<
  typeof GlazeWMComponentConfigSchema
>;
