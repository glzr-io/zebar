import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const KomorebiProviderConfigSchema = z.object({
  type: z.literal(ProviderType.KOMOREBI),
});

export type KomorebiProviderConfig = z.infer<
  typeof KomorebiProviderConfigSchema
>;
