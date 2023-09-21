import { z } from 'zod';

export const WindowTitleProviderConfigSchema = z.object({
  type: z.literal('window_title'),
});

export type WindowTitleProviderConfig = z.infer<
  typeof WindowTitleProviderConfigSchema
>;
