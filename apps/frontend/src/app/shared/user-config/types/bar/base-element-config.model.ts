import { z } from 'zod';

import { Prettify, createUniqueId } from '~/shared/utils';
import { ProviderConfigSchema } from './provider-config.model';

export const BaseElementConfigSchema = z.object({
  id: z.string().default(createUniqueId),
  class_name: z.string(),
  styles: z.string().optional(),
  providers: z
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
    .default([]),
});

/** Base config for bar, groups, and components. */
export type BaseElementConfig = Prettify<
  z.infer<typeof BaseElementConfigSchema>
>;
