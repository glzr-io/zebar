import { z } from 'zod';

import { ComponentGroupConfig } from './component-group-config.model';
import { Prettify } from '~/shared/utils';

export const BarConfig = z.intersection(
  z.object({
    group: ComponentGroupConfig.optional(),
  }),
  z.record(z.string().startsWith('group/'), ComponentGroupConfig),
);

export type BarConfig = Prettify<z.infer<typeof BarConfig>>;
