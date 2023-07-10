import { z } from 'zod';

import { BarConfig } from './bar/bar-config.model';
import { GeneralConfig } from './general-config.model';
import { Prettify } from '~/shared/utils';

const getKey = z.custom<`bar/${string}`>(val => {
  return (val as string).startsWith('bar/');
});

export const UserConfig = z
  .object({
    general: GeneralConfig.optional(),
    bar: BarConfig.optional(),
  })
  .and(z.record(getKey, BarConfig).optional());

export type UserConfig = Prettify<z.infer<typeof UserConfig>>;
