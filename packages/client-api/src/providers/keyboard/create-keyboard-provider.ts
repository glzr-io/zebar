import { z } from 'zod';

import { createBaseProvider } from '../create-base-provider';
import { onProviderEmit } from '~/desktop';
import type {
  KeyboardOutput,
  KeyboardProvider,
  KeyboardProviderConfig,
} from './keyboard-provider-types';

const keyboardProviderConfigSchema = z.object({
  type: z.literal('keyboard'),
  refreshInterval: z.coerce.number().default(1000),
});

export function createKeyboardProvider(
  config: KeyboardProviderConfig,
): KeyboardProvider {
  const mergedConfig = keyboardProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<KeyboardOutput>(mergedConfig, ({ result }) => {
      if ('error' in result) {
        queue.error(result.error);
      } else {
        queue.output(result.output);
      }
    });
  });
}
