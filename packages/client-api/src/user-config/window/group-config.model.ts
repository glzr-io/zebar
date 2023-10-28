import { z } from 'zod';

import { TemplateConfigSchema } from './template-config.model';
import { BaseElementConfigSchema } from './base-element-config.model';
import { Prettify } from '~/utils';
import { withDynamicKey } from '../shared';

export const GroupConfigSchemaP1 = BaseElementConfigSchema.extend({
  class_name: z.string().default('group'),
});

export type GroupConfigP1 = Prettify<z.infer<typeof GroupConfigSchemaP1>>;

// Add `template/**` keys to schema.
export const GroupConfigSchema = withDynamicKey(GroupConfigSchemaP1, {
  isKey: (key: string): key is `template/${string}` =>
    key.startsWith('template/'),
  schema: TemplateConfigSchema,
});

export type GroupConfig = Prettify<z.infer<typeof GroupConfigSchema>>;
