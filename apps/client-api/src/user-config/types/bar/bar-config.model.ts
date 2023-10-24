import { z } from 'zod';

import { GroupConfigSchema } from './group-config.model';
import { BaseElementConfigSchema } from './base-element-config.model';
import { withDynamicKey } from '../shared/with-dynamic-key';
import { Prettify } from '../../../utils';
import { BooleanLikeSchema } from '../shared/boolean-like.model';

export const BarConfigSchemaP1 = BaseElementConfigSchema.extend({
  class_name: z.string().default('bar'),
  position_x: z.coerce.number(),
  position_y: z.coerce.number(),
  width: z.coerce.number().min(1),
  height: z.coerce.number().min(1),
  always_on_top: BooleanLikeSchema.optional(),
  show_in_taskbar: BooleanLikeSchema.optional(),
  resizable: BooleanLikeSchema.optional(),
});

export type BarConfigP1 = Prettify<z.infer<typeof BarConfigSchemaP1>>;

// Add `group/**` keys to schema.
export const BarConfigSchema = withDynamicKey(BarConfigSchemaP1, {
  isKey: (key: string): key is `group/${string}` => key.startsWith('group/'),
  schema: GroupConfigSchema,
});

export type BarConfig = Prettify<z.infer<typeof BarConfigSchema>>;
