import { z } from 'zod';

export const SystemTrayProviderConfigSchema = z.object({
  type: z.literal('system_tray'),
});

export type SystemTrayProviderConfig = z.infer<
  typeof SystemTrayProviderConfigSchema
>;
