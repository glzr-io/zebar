import { z } from 'zod';

import {
  createBaseProvider,
  type Provider,
} from '../create-base-provider';
import { onProviderEmit } from '~/desktop';

export interface KeyboardProviderConfig {
  type: 'keyboard';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

const keyboardProviderConfigSchema = z.object({
  type: z.literal('keyboard'),
  refreshInterval: z.coerce.number().default(1000),
});

export type KeyboardProvider = Provider<
  KeyboardProviderConfig,
  KeyboardOutput
>;

export interface KeyboardOutput {
  layout: string;
}

export async function createKeyboardProvider(
  config: KeyboardProviderConfig,
): Promise<KeyboardProvider> {
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
