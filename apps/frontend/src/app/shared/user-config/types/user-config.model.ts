import { z } from 'zod';

import { BarConfigSchema } from './bar/bar-config.model';
import { GlobalConfigSchema } from './global-config.model';
import { withDynamicKey } from './shared/with-dynamic-key';
import { Prettify } from '~/shared/utils';

export const UserConfigP1Schema = z.object({
  global: GlobalConfigSchema,
});

export type UserConfigP1 = Prettify<z.infer<typeof UserConfigP1Schema>>;

// Add `bar/**` keys to schema.
export const UserConfigSchema = withDynamicKey(UserConfigP1Schema, {
  isKey: (key: string): key is `bar/${string}` => key.startsWith('bar/'),
  schema: BarConfigSchema,
});

export type UserConfig = Prettify<z.infer<typeof UserConfigSchema>>;
