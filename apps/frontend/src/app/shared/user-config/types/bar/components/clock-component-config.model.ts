import { z } from 'zod';

import { ElementSchema } from '../../shared/element.model';

export const ClockComponentConfigSchema = ElementSchema.extend({
  type: z.literal('clock'),
});

export type ClockComponentConfig = z.infer<typeof ClockComponentConfigSchema>;
