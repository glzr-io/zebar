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

const iconCache = new Map<string, { iconUrl: string, iconBlob: Blob }>();

function hashIconBytes(bytes: number[]): string {
  const length = bytes.length;
  const sample = [
    ...bytes.slice(0, 8),
    length,
    ...bytes.slice(length - 8, length)
  ];

  return sample.join(',');
}

export function createSystrayProvider(
  config: SystrayProviderConfig,
): SystrayProvider {
  const mergedConfig = systrayProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<SystrayOutput>(
      mergedConfig,
      ({ configHash, result }) => {
        if ('error' in result) {
          queue.error(result.error);
        } else {
          queue.output({
            ...result.output,
            icons: result.output.icons.map(icon => {
              const iconHash = hashIconBytes(icon.iconBytes);
              let cached = iconCache.get(iconHash);
              if (!cached) {
                const iconBlob = new Blob([new Uint8Array(icon.iconBytes)], {
                  type: 'image/png',
                });
                const iconUrl = URL.createObjectURL(iconBlob);
                cached = { iconUrl, iconBlob };
                iconCache.set(iconHash, cached);
              }

              return {
                ...icon,
                iconBlob: cached.iconBlob,
                iconUrl: cached.iconUrl,
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
        }
      },
    );
  });
}
