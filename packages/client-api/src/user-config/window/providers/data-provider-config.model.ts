import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const DataProviderConfigSchema = z.object({
  type: z.literal(ProviderType.DATA),
});

export type DataProviderConfig = z.infer<typeof DataProviderConfigSchema>;
