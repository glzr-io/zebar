import { z } from 'zod';

import { WindowConfigSchema } from './window/window-config.model';
import { GlobalConfigSchema } from './global-config.model';
import { withDynamicKey } from './shared/with-dynamic-key';
import { Prettify } from '~/utils';

export const UserConfigP1Schema = z.object({
  global: GlobalConfigSchema,
});

export type UserConfigP1 = Prettify<z.infer<typeof UserConfigP1Schema>>;

// Add `window/**` keys to schema.
export const UserConfigSchema = withDynamicKey(UserConfigP1Schema, {
  isKey: (key: string): key is `window/${string}` => key.startsWith('window/'),
  schema: WindowConfigSchema,
});

export type UserConfig = Prettify<z.infer<typeof UserConfigSchema>>;
