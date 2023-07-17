import { z } from 'zod';

import { GroupConfigSchema } from './group-config.model';
import { addDelimitedKey } from '../shared/add-delimited-key';
import { Prettify, ExcludeIndexedKeys } from '~/shared/utils';

export const BarConfigSchema = z
  .object({
    class_name: z.string().default('bar'),
    group: GroupConfigSchema.optional(),
  })
  .passthrough()
  .superRefine(addDelimitedKey('group', GroupConfigSchema.optional()))
  .refine((v): v is ExcludeIndexedKeys<typeof v> => true);

export type BarConfig = Prettify<z.infer<typeof BarConfigSchema>>;
