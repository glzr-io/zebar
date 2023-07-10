import { z } from 'zod';

import { BarConfig } from './bar/bar-config.model';
import { GeneralConfig } from './general-config.model';
import { Prettify } from '~/shared/utils';

export const UserConfig = z.intersection(
  z.object({
    general: GeneralConfig.optional(),
    bar: BarConfig.optional(),
  }),
  z
    .record(z.string().startsWith('bar/'), BarConfig)
    .optional()
    .refine((_): _ is { [key: `bar/${string}`]: BarConfig } => true),
);

export type UserConfig = Prettify<z.infer<typeof UserConfig>>;
