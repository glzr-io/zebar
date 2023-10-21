import { z } from 'zod';

export const MemoryProviderOptionsSchema = z.object({
  refresh_interval_ms: z.coerce.number().default(5 * 1000),
});

export type MemoryProviderOptions = z.infer<typeof MemoryProviderOptionsSchema>;

export const MemoryProviderConfigSchema = MemoryProviderOptionsSchema.extend({
  type: z.literal('memory'),
});

export type MemoryProviderConfig = z.infer<typeof MemoryProviderConfigSchema>;
