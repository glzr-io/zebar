import { z } from 'zod';

import { BarConfig, BarConfigSchema } from './bar/bar-config.model';
import { GeneralConfigSchema } from './general-config.model';
import { Prettify } from '~/shared/utils';

const UserConfigSchema1 = z.object({
  general: GeneralConfigSchema,
  bar: BarConfigSchema.optional(),
});

type UserConfig1 = z.infer<typeof UserConfigSchema1>;

export const UserConfigSchema = UserConfigSchema1.passthrough().superRefine(
  (arg, ctx): arg is UserConfig1 & { [key: `bar/${string}`]: BarConfig } => {
    const barKeys = Object.keys(arg).filter(key => key.startsWith('bar/'));

    for (const key of barKeys) {
      const res = BarConfigSchema.safeParse(arg[key]);

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

export type UserConfig = Prettify<z.infer<typeof UserConfigSchema>>;
