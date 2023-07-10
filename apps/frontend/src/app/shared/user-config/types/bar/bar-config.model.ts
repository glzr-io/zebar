import { z } from 'zod';

import { ComponentGroupConfigSchema } from './component-group-config.model';
import { delimitedKey } from '../../delimited-key';
import { Prettify } from '~/shared/utils';

export const BarConfigSchema = z
  .object({
    group: ComponentGroupConfigSchema.optional(),
  })
  .and(z.record(delimitedKey('group'), ComponentGroupConfigSchema).optional());

export type BarConfig = Prettify<z.infer<typeof BarConfigSchema>>;
