import { z } from 'zod';

export const GeneralConfigSchema = z
  .object({
    position_x: z.string(),
    position_y: z.string(),
    width: z.string(),
    height: z.string(),
    opacity: z.number(),
    enable_devtools: z.boolean(),
    enable_default_styles: z.boolean(),
    global_styles: z.string(),
    global_stylesheet_path: z.string(),
  })
  .partial();

export type GeneralConfig = z.infer<typeof GeneralConfigSchema>;