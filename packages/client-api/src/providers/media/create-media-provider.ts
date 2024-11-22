import { z } from 'zod';
import { createBaseProvider } from '../create-base-provider';
import { desktopCommands, onProviderEmit } from '~/desktop';
import type {
  MediaControlArgs,
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
            play: (args?: MediaControlArgs) => {
              desktopCommands.callProviderFunction(configHash, {
                type: 'media',
                function: {
                  name: 'play',
                  args: args ?? {},
                },
              });
            },
            pause: (args?: MediaControlArgs) => {
              desktopCommands.callProviderFunction(configHash, {
                type: 'media',
                function: {
                  name: 'pause',
                  args: args ?? {},
                },
              });
            },
            togglePlayPause: (args?: MediaControlArgs) => {
              desktopCommands.callProviderFunction(configHash, {
                type: 'media',
                function: {
                  name: 'toggle_play_pause',
                  args: args ?? {},
                },
              });
            },
            next: (args?: MediaControlArgs) => {
              desktopCommands.callProviderFunction(configHash, {
                type: 'media',
                function: {
                  name: 'next',
                  args: args ?? {},
                },
              });
            },
            previous: (args?: MediaControlArgs) => {
              desktopCommands.callProviderFunction(configHash, {
                type: 'media',
                function: {
                  name: 'previous',
                  args: args ?? {},
                },
              });
            },
          });
        }
      },
    );
  });
}
