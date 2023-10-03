import { z } from 'zod';

export const MemoryProviderConfigSchema = z.object({
  type: z.literal('memory'),
  refresh_interval_ms: z.coerce.number().default(5 * 1000),
});

export type MemoryProviderConfig = z.infer<typeof MemoryProviderConfigSchema>;
