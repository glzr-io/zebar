import { z } from 'zod';

import { BaseElementConfigSchema } from './base-element-config.model';
import { withDynamicKey } from '../shared/with-dynamic-key';

export const ComponentConfigSchemaP1 = BaseElementConfigSchema.extend({
  providers: z.string(),
  template: z.string(),
  slot: z.string().optional(),
});

// Add `slot/**` keys to schema.
export const ComponentConfigSchema = withDynamicKey(ComponentConfigSchemaP1, {
  isKey: (key: string): key is `slot/${string}` => key.startsWith('slot/'),
  schema: z.string(),
});

export type ComponentConfig = z.infer<typeof ComponentConfigSchema>;
