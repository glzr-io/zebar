import { z } from 'zod';

import { ComponentConfigSchema } from './component-config.model';
import { BaseElementConfigSchema } from './base-element-config.model';
import { Prettify } from '~/utils';
import { withDynamicKey } from '../shared/with-dynamic-key';

export const GroupConfigSchemaP1 = BaseElementConfigSchema.extend({
  class_name: z.string().default('group'),
});

export type GroupConfigP1 = Prettify<z.infer<typeof GroupConfigSchemaP1>>;

// Add `component/**` keys to schema.
export const GroupConfigSchema = withDynamicKey(GroupConfigSchemaP1, {
  isKey: (key: string): key is `component/${string}` =>
    key.startsWith('component/'),
  schema: ComponentConfigSchema,
});

export type GroupConfig = Prettify<z.infer<typeof GroupConfigSchema>>;
