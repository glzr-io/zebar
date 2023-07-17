import { z } from 'zod';

import { GroupConfigSchema } from './group-config.model';
import { addDelimitedKey } from '../shared/add-delimited-key';
import { TemplateElementConfigSchema } from '../shared/template-element-config.model';
import { Prettify } from '~/shared/utils';

export const BarConfigSchema = TemplateElementConfigSchema.extend({
  class_name: z.string().default('bar'),
  group: GroupConfigSchema.optional(),
})
  .passthrough()
  .superRefine(addDelimitedKey('group', GroupConfigSchema));

export type BarConfig = Prettify<z.infer<typeof BarConfigSchema>>;
