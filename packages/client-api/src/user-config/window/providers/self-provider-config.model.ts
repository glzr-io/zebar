import { z } from 'zod';

export const SelfProviderConfigSchema = z.object({
  type: z.literal('self'),
});

export type SelfProviderConfig = z.infer<typeof SelfProviderConfigSchema>;
