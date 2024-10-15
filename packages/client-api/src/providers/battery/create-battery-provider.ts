import { z } from 'zod';

import { createBaseProvider } from '../create-base-provider';
import { onProviderEmit } from '~/desktop';
import type {
  BatteryOutput,
  BatteryProvider,
  BatteryProviderConfig,
} from './battery-provider-types';

const batteryProviderConfigSchema = z.object({
  type: z.literal('battery'),
  refreshInterval: z.coerce.number().default(60 * 1000),
});

export function createBatteryProvider(
  config: BatteryProviderConfig,
): BatteryProvider {
  const mergedConfig = batteryProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<BatteryOutput>(mergedConfig, ({ result }) => {
      if ('error' in result) {
        queue.error(result.error);
      } else {
        queue.output(result.output);
      }
    });
  });
}
