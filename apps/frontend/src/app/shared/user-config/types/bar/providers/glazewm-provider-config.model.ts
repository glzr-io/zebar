import { z } from 'zod';

export const GlazeWMProviderConfigSchema = z.object({
  type: z.literal('glazewm'),
});

export type GlazeWMProviderConfig = z.infer<typeof GlazeWMProviderConfigSchema>;
