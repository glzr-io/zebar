import { z } from 'zod';

import { GroupConfigSchema } from './group-config.model';
import { addDelimitedKey } from '../shared/add-delimited-key';
import { ElementSchema } from '../shared/element.model';
import { Prettify } from '~/shared/utils';

export const BarConfigSchema = ElementSchema.extend({
  class_name: z.string().default('bar'),
  group: GroupConfigSchema.optional(),
})
  .passthrough()
  .superRefine(addDelimitedKey('group', GroupConfigSchema));

export type BarConfig = Prettify<z.infer<typeof BarConfigSchema>>;
