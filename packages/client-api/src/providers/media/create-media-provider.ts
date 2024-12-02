import { z } from 'zod';
import { createBaseProvider } from '../create-base-provider';
import { desktopCommands, onProviderEmit } from '~/desktop';
import type {
  MediaControlOptions,
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
            session: result.output.currentSession,
            play: (options?: MediaControlOptions) => {
              return desktopCommands.callProviderFunction(configHash, {
                type: 'media',
                function: {
                  name: 'play',
                  args: options ?? {},
                },
              });
            },
            pause: (options?: MediaControlOptions) => {
              return desktopCommands.callProviderFunction(configHash, {
                type: 'media',
                function: {
                  name: 'pause',
                  args: options ?? {},
                },
              });
            },
            togglePlayPause: (options?: MediaControlOptions) => {
              return desktopCommands.callProviderFunction(configHash, {
                type: 'media',
                function: {
                  name: 'toggle_play_pause',
                  args: options ?? {},
                },
              });
            },
            next: (options?: MediaControlOptions) => {
              return desktopCommands.callProviderFunction(configHash, {
                type: 'media',
                function: {
                  name: 'next',
                  args: options ?? {},
                },
              });
            },
            previous: (options?: MediaControlOptions) => {
              return desktopCommands.callProviderFunction(configHash, {
                type: 'media',
                function: {
                  name: 'previous',
                  args: options ?? {},
                },
              });
            },
          });
        }
      },
    );
  });
}
