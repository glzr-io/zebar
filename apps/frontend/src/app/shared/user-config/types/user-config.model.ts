import { z } from 'zod';

import { BarConfigSchema } from './bar/bar-config.model';
import { GeneralConfigSchema } from './general-config.model';
import { withDynamicKey } from './shared/with-dynamic-key';
import { Prettify } from '~/shared/utils';

const UserConfigSchemaP1 = z.object({
  general: GeneralConfigSchema,
  bar: BarConfigSchema.optional(),
});

export const UserConfigSchema = withDynamicKey(UserConfigSchemaP1, {
  isKey: (key: string): key is `bar/${string}` => key.startsWith('bar/'),
  schema: BarConfigSchema,
});

export type UserConfig = Prettify<z.infer<typeof UserConfigSchema>>;
