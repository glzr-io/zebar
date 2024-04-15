import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const DataProviderConfigSchema = z.object({
  type: z.literal(ProviderType.DATA),

  refresh_interval: z.coerce.number().default(1000),
});

export type DataProviderConfig = z.infer<typeof DataProviderConfigSchema>;
