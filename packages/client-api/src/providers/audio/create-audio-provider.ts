import { z } from 'zod';

import { createBaseProvider } from '../create-base-provider';
import { desktopCommands, onProviderEmit } from '~/desktop';
import type {
  AudioOutput,
  AudioProvider,
  AudioProviderConfig,
  SetVolumeOptions,
} from './audio-provider-types';

const audioProviderConfigSchema = z.object({
  type: z.literal('audio'),
});

export function createAudioProvider(
  config: AudioProviderConfig,
): AudioProvider {
  const mergedConfig = audioProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<AudioOutput>(
      mergedConfig,
      ({ configHash, result }) => {
        if ('error' in result) {
          queue.error(result.error);
        } else {
          queue.output({
            ...result.output,
            setVolume: (volume: number, options?: SetVolumeOptions) => {
              return desktopCommands.callProviderFunction(configHash, {
                type: 'audio',
                function: {
                  name: 'set_volume',
                  args: { volume, deviceId: options?.deviceId },
                },
              });
            },
          });
        }
      },
    );
  });
}
