import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const MemoryProviderConfigSchema = z.object({
  type: z.literal(ProviderType.MEMORY),

  refresh_interval_ms: z.coerce.number().default(5 * 1000),
});

export type MemoryProviderConfig = z.infer<
  typeof MemoryProviderConfigSchema
>;
