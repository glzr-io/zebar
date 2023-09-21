import { z } from 'zod';

export const CustomProviderConfigSchema = z.object({
  type: z.literal('custom'),
});

export type CustomProviderConfig = z.infer<typeof CustomProviderConfigSchema>;
