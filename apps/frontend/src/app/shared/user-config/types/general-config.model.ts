import { z } from 'zod';

import { BooleanLikeSchema } from './shared/boolean-like.model';

export const GeneralConfigSchema = z
  .object({
    enable_devtools: BooleanLikeSchema,
    root_styles: z.string(),
    load_stylesheets: z.array(z.string()),
    load_scripts: z.array(z.string()),
  })
  .partial();

export type GeneralConfig = z.infer<typeof GeneralConfigSchema>;
