import { z } from 'zod';

import { BarConfig } from './bar/bar-config.model';
import { GeneralConfig } from './general-config.model';

export const UserConfig = z.object({
  general: GeneralConfig,
  bar: z.record(z.string().startsWith('bar/'), z.string()),
});

export type UserConfig = z.infer<typeof UserConfig>;
