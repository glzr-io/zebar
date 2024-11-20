import { z } from 'zod';

import { createBaseProvider } from '../create-base-provider';
import { onProviderEmit } from '~/desktop';
import type {
  AudioOutput,
  AudioProvider,
  AudioProviderConfig,
} from './audio-provider-types';

const audioProviderConfigSchema = z.object({
  type: z.literal('audio'),
});

export function createAudioProvider(
  config: AudioProviderConfig,
): AudioProvider {
  const mergedConfig = audioProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<AudioOutput>(mergedConfig, ({ result }) => {
      if ('error' in result) {
        queue.error(result.error);
      } else {
        queue.output(result.output);
      }
    });
  });
}
