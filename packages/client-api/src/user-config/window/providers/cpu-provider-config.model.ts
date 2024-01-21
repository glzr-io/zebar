import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const CpuProviderConfigSchema = z.object({
  type: z.literal(ProviderType.CPU),

  refresh_interval_ms: z.coerce.number().default(5 * 1000),
});

export type CpuProviderConfig = z.infer<typeof CpuProviderConfigSchema>;
