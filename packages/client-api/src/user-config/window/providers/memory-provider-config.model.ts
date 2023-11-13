import { z } from 'zod';

export const MemoryProviderConfigSchema = z.object({
  refresh_interval_ms: z.coerce.number().default(5 * 1000),
  type: z.literal('memory'),
});

export type MemoryProviderConfig = z.infer<typeof MemoryProviderConfigSchema>;
