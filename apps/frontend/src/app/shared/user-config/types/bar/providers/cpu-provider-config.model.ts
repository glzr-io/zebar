import { z } from 'zod';

export const CpuProviderOptionsSchema = z.object({
  refresh_interval_ms: z.coerce.number().default(5 * 1000),
});

export type CpuProviderOptions = z.infer<typeof CpuProviderOptionsSchema>;

export const CpuProviderConfigSchema = CpuProviderOptionsSchema.extend({
  type: z.literal('cpu'),
});

export type CpuProviderConfig = z.infer<typeof CpuProviderConfigSchema>;
