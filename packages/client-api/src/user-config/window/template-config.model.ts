import { z } from 'zod';

import { BaseElementConfigSchema } from './base-element-config.model';
import { withDynamicKey } from '../shared';

export const TemplateConfigSchemaP1 = BaseElementConfigSchema.extend({
  class_name: z.string().default('template'),
  template: z.string(),
  slot: z.string().optional(),
});

// Add `slot/**` keys to schema.
export const TemplateConfigSchema = withDynamicKey(
  TemplateConfigSchemaP1,
  {
    isKey: (key: string): key is `slot/${string}` =>
      key.startsWith('slot/'),
    schema: z.string(),
  },
);

export type TemplateConfig = z.infer<typeof TemplateConfigSchema>;
