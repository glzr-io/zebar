import { z } from 'zod';

import { BarConfigSchema } from './bar/bar-config.model';
import { GeneralConfigSchema } from './general-config.model';
import { Prettify } from '~/shared/utils';
import { delimitedKey } from '../delimited-key';

export const UserConfigSchema = z
  .object({
    general: GeneralConfigSchema.optional(),
    bar: BarConfigSchema.optional(),
  })
  .and(z.record(delimitedKey('bar'), BarConfigSchema).optional());

export type UserConfig = Prettify<z.infer<typeof UserConfigSchema>>;
