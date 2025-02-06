import { z } from 'zod';
import { createBaseProvider } from '../create-base-provider';
import { onProviderEmit } from '~/desktop';
import type {
  FocusedWindowOutput,
  FocusedWindowProvider,
  FocusedWindowProviderConfig,
} from './focused-window-provider-types';

const focusedWindowProviderConfigSchema = z.object({
  type: z.literal('focusedWindow'),
});

export function createFocusedWindowProvider(
  config: FocusedWindowProviderConfig,
): FocusedWindowProvider {
  const mergedConfig = focusedWindowProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<FocusedWindowOutput>(
      mergedConfig,
      ({ result }) => {
        if ('error' in result) {
          queue.error(result.error);
        } else {
          const iconBlob = new Blob(
            [new Uint8Array(result.output.iconBytes)],
            {
              type: 'image/png',
            },
          );
          queue.output({
            ...result.output,
            iconBlob,
            iconURL: URL.createObjectURL(iconBlob),
          });
        }
      },
    );
  });
}
