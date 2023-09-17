import { z } from 'zod';

import { GroupConfigSchema } from './group-config.model';
import { TemplateElementConfigSchema } from '../shared/template-element-config.model';
import { withDynamicKey } from '../shared/with-dynamic-key';
import { Prettify } from '~/shared/utils';

const BarConfigSchemaP1 = TemplateElementConfigSchema.extend({
  class_name: z.string().default('bar'),
  position_x: z.string(),
  position_y: z.string(),
  width: z.string(),
  height: z.string(),
  always_on_top: z.boolean(),
  show_in_taskbar: z.boolean(),
  resizable: z.boolean(),
});

// Add `group/**` keys to schema.
export const BarConfigSchema = withDynamicKey(BarConfigSchemaP1, {
  isKey: (key: string): key is `group/${string}` => key.startsWith('group/'),
  schema: GroupConfigSchema,
});

export type BarConfig = Prettify<z.infer<typeof BarConfigSchema>>;
