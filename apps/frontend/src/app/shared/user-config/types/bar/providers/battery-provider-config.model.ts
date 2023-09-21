import { z } from 'zod';

export const BatteryProviderConfigSchema = z.object({
  type: z.literal('battery'),
  refresh_interval_ms: z.number().default(60 * 1000),
});

export type BatteryProviderConfig = z.infer<typeof BatteryProviderConfigSchema>;
