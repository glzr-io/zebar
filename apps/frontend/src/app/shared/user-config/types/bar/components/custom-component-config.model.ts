import { z } from 'zod';

import { ComponentConfigBaseSchema } from '../component-config-base.model';
import { withSlotSchema } from '../with-slot-schema';

export const CustomComponentConfigSchema = withSlotSchema(
  ComponentConfigBaseSchema.extend({
    type: z.literal('custom'),
    class_name: z.string().default('custom-component'),
  }),
);

export type CustomComponentConfig = z.infer<typeof CustomComponentConfigSchema>;
