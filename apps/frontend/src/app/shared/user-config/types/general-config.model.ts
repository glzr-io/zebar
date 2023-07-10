import { z } from 'zod';

export const GeneralConfig = z
  .object({
    positionX: z.string(),
    positionY: z.string(),
    width: z.string(),
    height: z.string(),
    opacity: z.number(),
    enableDevtools: z.boolean(),
    enableDefaultStyles: z.boolean(),
    globalStyles: z.string(),
    globalStylesheetPath: z.string(),
  })
  .partial();

export type GeneralConfig = z.infer<typeof GeneralConfig>;
