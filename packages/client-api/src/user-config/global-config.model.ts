import { z } from 'zod';

import { BooleanLikeSchema } from './shared';

export const GlobalConfigSchema = z
  .object({
    enable_devtools: BooleanLikeSchema.optional(),
  })
  .partial();

export type GlobalConfig = z.infer<typeof GlobalConfigSchema>;
