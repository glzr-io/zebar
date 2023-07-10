import { z } from 'zod';

import { BarConfig } from './bar/bar-config.model';
import { GeneralConfig } from './general-config.model';
import { Prettify } from '~/shared/utils';
import { delimitedKey } from '../delimited-key';

export const UserConfig = z
  .object({
    general: GeneralConfig.optional(),
    bar: BarConfig.optional(),
  })
  .and(z.record(delimitedKey('bar'), BarConfig).optional());

export type UserConfig = Prettify<z.infer<typeof UserConfig>>;
