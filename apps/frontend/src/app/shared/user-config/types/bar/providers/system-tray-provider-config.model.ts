import { z } from 'zod';

export const SystemTrayProviderOptionsSchema = z.object({});

export type SystemTrayProviderOptions = z.infer<
  typeof SystemTrayProviderOptionsSchema
>;

export const SystemTrayProviderConfigSchema =
  SystemTrayProviderOptionsSchema.extend({
    type: z.literal('system_tray'),
  });

export type SystemTrayProviderConfig = z.infer<
  typeof SystemTrayProviderConfigSchema
>;
