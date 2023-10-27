import { z } from 'zod';

import { ProviderConfigSchema } from './provider-config.model';
import { Prettify } from '~/utils';

export const ProvidersConfigSchema = z
  .array(
    z.union([
      ProviderConfigSchema,
      z
        .enum([
          'active_window',
          'battery',
          'cpu',
          'custom',
          'date_time',
          'glazewm',
          'ip',
          'memory',
          'network',
          'system_tray',
          'weather',
        ])
        .transform(type => ProviderConfigSchema.parse({ type })),
    ]),
  )
  .default([]);

export type ProvidersConfig = Prettify<z.infer<typeof ProvidersConfigSchema>>;
