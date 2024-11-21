import { z } from 'zod';
import { createBaseProvider } from '../create-base-provider';
import { desktopCommands, onProviderEmit } from '~/desktop';
import type {
  MediaOutput,
  MediaProvider,
  MediaProviderConfig,
} from './media-provider-types';

const mediaProviderConfigSchema = z.object({
  type: z.literal('media'),
});

export function createMediaProvider(
  config: MediaProviderConfig,
): MediaProvider {
  const mergedConfig = mediaProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<MediaOutput>(
      mergedConfig,
      ({ result, configHash }) => {
        if ('error' in result) {
          queue.error(result.error);
        } else {
          queue.output({
            ...result.output,
            play: () => {
              desktopCommands.callProviderFunction({
                configHash,
                function: { type: 'media', function: 'play' },
              });
            },
            pause: () => {
              desktopCommands.callProviderFunction({
                configHash,
                function: { type: 'media', function: 'pause' },
              });
            },
            togglePlayPause: () => {
              desktopCommands.callProviderFunction({
                configHash,
                function: { type: 'media', function: 'toggle_play_pause' },
              });
            },
            next: () => {
              desktopCommands.callProviderFunction({
                configHash,
                function: { type: 'media', function: 'next' },
              });
            },
            previous: () => {
              desktopCommands.callProviderFunction({
                configHash,
                function: { type: 'media', function: 'previous' },
              });
            },
          });
        }
      },
    );
  });
}
