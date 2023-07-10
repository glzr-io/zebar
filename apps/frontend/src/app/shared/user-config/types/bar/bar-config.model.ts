import { z } from 'zod';

import { ComponentGroupConfig } from './component-group-config.model';
import { delimitedKey } from '../../delimited-key';
import { Prettify } from '~/shared/utils';

export const BarConfig = z
  .object({
    group: ComponentGroupConfig.optional(),
  })
  .and(z.record(delimitedKey('group'), ComponentGroupConfig).optional());

export type BarConfig = Prettify<z.infer<typeof BarConfig>>;
