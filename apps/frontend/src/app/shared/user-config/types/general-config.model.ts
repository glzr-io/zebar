import { z } from 'zod';

export const GeneralConfigSchema = z
  .object({
    position_x: z.string(),
    position_y: z.string(),
    width: z.string(),
    height: z.string(),
    transparent: z.boolean(),
    enable_devtools: z.boolean(),
    global_styles: z.string(),
    global_stylesheet_path: z.string(),
  })
  .partial();

export type GeneralConfig = z.infer<typeof GeneralConfigSchema>;
