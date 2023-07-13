import { z } from 'zod';

import {
  ComponentGroupConfig,
  ComponentGroupConfigSchema,
} from './component-group-config.model';
import { Prettify } from '~/shared/utils';
import { ElementSchema } from '../shared/element.model';

const BarConfigSchema1 = ElementSchema.extend({
  class_name: z.string().default('bar'),
  group: ComponentGroupConfigSchema.optional(),
});

type BarConfig1 = z.infer<typeof BarConfigSchema1>;

export const BarConfigSchema = BarConfigSchema1.passthrough().superRefine(
  (
    arg,
    ctx,
  ): arg is BarConfig1 & { [key: `group/${string}`]: ComponentGroupConfig } => {
    const groupKeys = Object.keys(arg).filter(key => key.startsWith('group/'));

    for (const key of groupKeys) {
      const res = ComponentGroupConfigSchema.safeParse(arg[key]);

      if (res.success) {
        arg[key] = res.data;
        continue;
      }

      for (const issue of res.error.issues) {
        ctx.addIssue({
          ...issue,
          path: [key, ...issue.path],
        });
      }
    }

    return z.NEVER;
  },
);

export type BarConfig = Prettify<z.infer<typeof BarConfigSchema>>;
