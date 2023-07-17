import { z } from 'zod';

import { TemplateElementConfigSchema } from '../shared/template-element-config.model';

export const ComponentConfigBaseSchema = TemplateElementConfigSchema.extend({
  type: z.string(),
  slot: z.string().optional(),
}).passthrough();

export type ComponentConfigBase = z.infer<typeof ComponentConfigBaseSchema>;
