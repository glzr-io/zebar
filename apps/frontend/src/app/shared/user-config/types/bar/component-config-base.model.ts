import { z } from 'zod';

import { ElementSchema } from '../shared/element.model';

export const ComponentConfigBaseSchema = ElementSchema.extend({
  type: z.string(),
  slot: z.string().optional(),
}).passthrough();

export type ComponentConfigBase = z.infer<typeof ComponentConfigBaseSchema>;
