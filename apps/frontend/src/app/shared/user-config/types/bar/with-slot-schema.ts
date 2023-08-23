import { z } from 'zod';

import { withDynamicKey } from '../shared/with-dynamic-key';

/**
 * Adds a `slot/` property to a component schema.
 */
export function withSlotSchema<T extends z.AnyZodObject>(schema: T) {
  return withDynamicKey(schema, {
    isKey: (key: string): key is `slot/${string}` => key.startsWith('slot/'),
    schema: z.string(),
  });
}
