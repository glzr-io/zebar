import { z } from 'zod';

import { ComponentConfigBaseSchema } from '../component-config-base.model';

export const NetworkComponentConfigSchema = ComponentConfigBaseSchema.extend({
  type: z.literal('network'),
  class_name: z.string().default('network-component'),
});

export type NetworkComponentConfig = z.infer<
  typeof NetworkComponentConfigSchema
>;
