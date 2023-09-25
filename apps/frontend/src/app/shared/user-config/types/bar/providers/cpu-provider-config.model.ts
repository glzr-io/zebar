import { z } from 'zod';

export const CpuProviderConfigSchema = z.object({
  type: z.literal('cpu'),
  refresh_interval_ms: z.number().default(5 * 1000),
});

export type CpuProviderConfig = z.infer<typeof CpuProviderConfigSchema>;
