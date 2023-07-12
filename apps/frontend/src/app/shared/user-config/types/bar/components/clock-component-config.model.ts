import { z } from 'zod';

import { ElementSchema } from '../../shared/element.model';

export const ClockComponentConfigSchema = ElementSchema.extend({
  type: z.literal('clock'),
  class_name: z.string().default('clock-component'),
});

export type ClockComponentConfig = z.infer<typeof ClockComponentConfigSchema>;
