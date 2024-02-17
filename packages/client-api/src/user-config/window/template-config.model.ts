import { z } from 'zod';

import { BaseElementConfigSchema } from './base-element-config.model';

export const TemplateConfigSchema = BaseElementConfigSchema.extend({
  class_names: z.array(z.string()).default(['template']),
  template: z.string(),
});

export type TemplateConfig = z.infer<typeof TemplateConfigSchema>;
