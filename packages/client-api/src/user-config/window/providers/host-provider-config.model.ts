import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const HostProviderConfigSchema = z.object({
  type: z.literal(ProviderType.HOST),

  refresh_interval_ms: z.coerce.number().default(60 * 1000),
});

export type HostProviderConfig = z.infer<typeof HostProviderConfigSchema>;
