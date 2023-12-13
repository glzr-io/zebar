import { z } from 'zod';

export const MonitorsProviderConfigSchema = z.object({
  type: z.literal('monitors'),
});

export type MonitorsProviderConfig = z.infer<
  typeof MonitorsProviderConfigSchema
>;
