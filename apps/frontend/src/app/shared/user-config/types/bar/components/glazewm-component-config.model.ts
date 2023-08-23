import { z } from 'zod';

import { ComponentConfigBaseSchema } from '../component-config-base.model';
import { withSlotSchema } from '../with-slot-schema';

export const GlazeWMComponentConfigSchema = withSlotSchema(
  ComponentConfigBaseSchema.extend({
    type: z.literal('glazewm'),
    class_name: z.string().default('glazewm-component'),
  }),
);

export type GlazeWMComponentConfig = z.infer<
  typeof GlazeWMComponentConfigSchema
>;
