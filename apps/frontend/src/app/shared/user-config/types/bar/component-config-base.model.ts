import { z } from 'zod';

import { addDelimitedKey } from '../shared/add-delimited-key';
import { ElementSchema } from '../shared/element.model';
import { ExcludeIndexedKeys, Prettify } from '~/shared/utils';

const SlotSchema = z.string().optional();

export const ComponentConfigBaseSchema = ElementSchema.extend({
  type: z.string(),
  slot: SlotSchema,
})
  .passthrough()
  .superRefine(addDelimitedKey('slot', SlotSchema))
  .refine((v): v is ExcludeIndexedKeys<typeof v> => true);

export type ComponentConfigBase = Prettify<
  z.infer<typeof ComponentConfigBaseSchema>
>;
