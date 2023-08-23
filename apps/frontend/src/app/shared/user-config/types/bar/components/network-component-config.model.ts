import { z } from 'zod';

import { ComponentConfigBaseSchema } from '../component-config-base.model';
import { withSlotSchema } from '../with-slot-schema';

export const NetworkComponentConfigSchema = withSlotSchema(
  ComponentConfigBaseSchema.extend({
    type: z.literal('network'),
    class_name: z.string().default('network-component'),
  }),
);

export type NetworkComponentConfig = z.infer<
  typeof NetworkComponentConfigSchema
>;
