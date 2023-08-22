import { z } from 'zod';

import { GroupConfigSchema } from './group-config.model';
import { TemplateElementConfigSchema } from '../shared/template-element-config.model';
import { withDynamicKey } from '../shared/with-dynamic-key';
import { Prettify } from '~/shared/utils';

const BarConfigSchemaP1 = TemplateElementConfigSchema.extend({
  class_name: z.string().default('bar'),
  group: GroupConfigSchema.optional(),
});

export const BarConfigSchema = withDynamicKey(BarConfigSchemaP1, {
  isKey: (key: string): key is `group/${string}` => key.startsWith('group/'),
  schema: GroupConfigSchema,
});

export type BarConfig = Prettify<z.infer<typeof BarConfigSchema>>;
