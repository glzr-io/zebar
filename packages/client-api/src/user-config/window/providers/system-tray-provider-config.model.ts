import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const SystemTrayProviderConfigSchema = z.object({
  type: z.literal(ProviderType.SYSTEM_TRAY),
});

export type SystemTrayProviderConfig = z.infer<
  typeof SystemTrayProviderConfigSchema
>;
