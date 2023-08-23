import { z } from 'zod';

import { ComponentConfigBaseSchema } from '../component-config-base.model';
import { withSlotSchema } from '../with-slot-schema';

export const SystemTrayComponentConfigSchema = withSlotSchema(
  ComponentConfigBaseSchema.extend({
    type: z.literal('system_tray'),
    class_name: z.string().default('system-tray-component'),
  }),
);

export type SystemTrayComponentConfig = z.infer<
  typeof SystemTrayComponentConfigSchema
>;
