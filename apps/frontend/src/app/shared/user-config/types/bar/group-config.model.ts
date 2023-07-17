import { z } from 'zod';

import { ComponentConfigSchema } from './component-config.model';
import { TemplateElementConfigSchema } from '../shared/template-element-config.model';

export const GroupConfigSchema = TemplateElementConfigSchema.extend({
  class_name: z.string().default('group'),
  components: z.array(ComponentConfigSchema).default([]),
});

export type GroupConfig = z.infer<typeof GroupConfigSchema>;
