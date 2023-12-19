import { z } from 'zod';

import { ProviderConfigSchema } from './provider-config.model';
import { ProviderTypeSchema } from './provider-type.model';
import { Prettify } from '~/utils';

export const ProvidersConfigSchema = z
  .array(
    z.union([
      ProviderConfigSchema,
      ProviderTypeSchema.transform(type =>
        ProviderConfigSchema.parse({ type }),
      ),
    ]),
  )
  .default([]);

export type ProvidersConfig = Prettify<
  z.infer<typeof ProvidersConfigSchema>
>;
