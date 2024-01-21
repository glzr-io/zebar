import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const SelfProviderConfigSchema = z.object({
  type: z.literal(ProviderType.SELF),
});

export type SelfProviderConfig = z.infer<typeof SelfProviderConfigSchema>;
