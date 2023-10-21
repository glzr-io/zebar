import { z } from 'zod';

import { BooleanLikeSchema } from './shared/boolean-like.model';

export const GlobalConfigSchema = z
  .object({
    enable_devtools: BooleanLikeSchema.optional(),
    root_styles: z.string().optional(),
    load_stylesheets: z.array(z.string()).default([]),
    load_scripts: z.array(z.string()).default([]),
  })
  .partial();

export type GlobalConfig = z.infer<typeof GlobalConfigSchema>;
