import { z } from 'zod';

import { createBaseProvider } from '../create-base-provider';
import { desktopCommands, onProviderEmit } from '~/desktop';
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

  // Cache icon blobs and object URLs to prevent flickering during updates.
  const iconCache = new Map<string, { iconBlob: Blob; iconUrl: string }>();

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<SystrayOutput>(
      mergedConfig,
      ({ configHash, result }) => {
        if ('error' in result) {
          queue.error(result.error);
        } else {
          // Collect hashes of all current icons to identify which cache
          // entries to keep.
          const currentHashes = new Set(
            result.output.icons.map(icon => icon.iconHash),
          );

          queue.output({
            ...result.output,
            icons: result.output.icons.map(icon => {
              let cachedIcon = iconCache.get(icon.iconHash);

              if (!cachedIcon) {
                // Create a new blob and object URL for this icon.
                const iconBlob = new Blob(
                  [new Uint8Array(icon.iconBytes)],
                  { type: 'image/png' },
                );

                cachedIcon = {
                  iconBlob,
                  iconUrl: URL.createObjectURL(iconBlob),
                };

                iconCache.set(icon.iconHash, cachedIcon);
              }

              return {
                ...icon,
                iconBlob: cachedIcon.iconBlob,
                iconUrl: cachedIcon.iconUrl,
              };
            }),
            onHoverEnter: (iconId: string) => {
              return desktopCommands.callProviderFunction(configHash, {
                type: 'systray',
                function: {
                  name: 'icon_hover_enter',
                  args: { iconId },
                },
              });
            },
            onHoverLeave: (iconId: string) => {
              return desktopCommands.callProviderFunction(configHash, {
                type: 'systray',
                function: {
                  name: 'icon_hover_leave',
                  args: { iconId },
                },
              });
            },
            onHoverMove: (iconId: string) => {
              return desktopCommands.callProviderFunction(configHash, {
                type: 'systray',
                function: {
                  name: 'icon_hover_move',
                  args: { iconId },
                },
              });
            },
            onLeftClick: (iconId: string) => {
              return desktopCommands.callProviderFunction(configHash, {
                type: 'systray',
                function: {
                  name: 'icon_left_click',
                  args: { iconId },
                },
              });
            },
            onRightClick: (iconId: string) => {
              return desktopCommands.callProviderFunction(configHash, {
                type: 'systray',
                function: {
                  name: 'icon_right_click',
                  args: { iconId },
                },
              });
            },
            onMiddleClick: (iconId: string) => {
              return desktopCommands.callProviderFunction(configHash, {
                type: 'systray',
                function: {
                  name: 'icon_middle_click',
                  args: { iconId },
                },
              });
            },
          });

          // Clean up cache to prevent leaking object URLs.
          for (const [hash, cachedIcon] of iconCache) {
            if (!currentHashes.has(hash)) {
              URL.revokeObjectURL(cachedIcon.iconUrl);
              iconCache.delete(hash);
            }
          }
        }
      },
    );
  });
}
