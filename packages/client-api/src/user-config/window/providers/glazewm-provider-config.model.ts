import { z } from 'zod';

export const GlazewmProviderConfigSchema = z.object({
  type: z.literal('glazewm'),
});

export type GlazewmProviderConfig = z.infer<
  typeof GlazewmProviderConfigSchema
>;
