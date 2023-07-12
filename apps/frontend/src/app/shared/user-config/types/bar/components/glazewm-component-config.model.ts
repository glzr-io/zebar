import { z } from 'zod';

import { ElementSchema } from '../../shared/element.model';

export const GlazeWMComponentConfigSchema = ElementSchema.extend({
  type: z.literal('glazewm'),
  class_name: z.string().default('glazewm-component'),
});

export type GlazeWMComponentConfig = z.infer<
  typeof GlazeWMComponentConfigSchema
>;
