import { z } from 'zod';

import { createBaseProvider } from '../create-base-provider';
import { onProviderEmit } from '~/desktop';
import type {
  SystrayOutput,
  SystrayProvider,
  SystrayProviderConfig,
} from './systray-provider-types';

const systrayProviderConfigSchema = z.object({
  type: z.literal('systray'),
});

export function createSystrayProvider(
  config: SystrayProviderConfig,
): SystrayProvider {
  const mergedConfig = systrayProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<SystrayOutput>(mergedConfig, ({ result }) => {
      if ('error' in result) {
        queue.error(result.error);
      } else {
        queue.output(result.output);
      }
    });
  });
}
