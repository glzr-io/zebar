import { z } from 'zod';

export const GeneralConfigSchema = z
  .object({
    enable_devtools: z.boolean(),
    global_styles: z.string(),
    global_stylesheet_path: z.string(),
  })
  .partial();

export type GeneralConfig = z.infer<typeof GeneralConfigSchema>;
