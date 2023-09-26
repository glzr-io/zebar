import { z } from 'zod';

export const GeneralConfigSchema = z
  .object({
    enable_devtools: z.boolean(),
    root_styles: z.string(),
    load_stylesheets: z.array(z.string()),
    load_scripts: z.array(z.string()),
  })
  .partial();

export type GeneralConfig = z.infer<typeof GeneralConfigSchema>;
