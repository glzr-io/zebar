import { z } from 'zod';

import { createBaseProvider } from '../create-base-provider';
import { onProviderEmit } from '~/desktop';
import type {
  WindowOutput,
  WindowProvider,
  WindowProviderConfig,
} from './window-provider-types';

const windowProviderConfigSchema = z.object({
  type: z.literal('window'),
  refreshInterval: z.coerce.number().default(1000),
});

export function createWindowProvider(config: WindowProviderConfig): WindowProvider {
  const mergedConfig = windowProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<WindowOutput>(mergedConfig, ({ result }) => {
      if ('error' in result) {
        queue.error(result.error);
      } else {
        queue.output(result.output);
      }
    });
  });
}