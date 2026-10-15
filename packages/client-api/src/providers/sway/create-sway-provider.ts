
import { z } from 'zod';

import { desktopCommands, getMonitors, onProviderEmit } from '../../desktop';
import { getCoordinateDistance } from '../../utils';
import { createBaseProvider } from '../create-base-provider';
import type {
  SwayState,
  SwayProvider,
  SwayProviderConfig, SwayResponse,
} from '../../providers/sway/sway-provider-types';

const SwayProviderConfigSchema = z.object({
  type: z.literal('sway'),
});


export function createSwayProvider(
  config: SwayProviderConfig,
): SwayProvider {
  const mergedConfig = SwayProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    const monitors = await getMonitors();

    const createState = async (output: SwayResponse, configHash: string) => {
      const currentMonitor = monitors.currentMonitor;

      const currentPosition = {
        x: monitors.currentMonitor!.x,
        y: monitors.currentMonitor!.y,
      };



      // Get GlazeWM monitor that corresponds to the widget's monitor.
      const currentOutput = output.allOutputs.reduce((a, b) =>
        getCoordinateDistance(currentPosition, a.rect) <
        getCoordinateDistance(currentPosition, b.rect)
          ? a
          : b,
      );
      return {
        ...output,
        currentWorkspaces: output.allWorkspaces.filter(x => x.output === currentOutput.name),
        currentOutput,
        runCommand(payload: string) {
          return desktopCommands.callProviderFunction(configHash, {
            type: 'sway',
            function: {
              name: 'run_command',
              payload
            }
          });
        },
      }
    };
    return onProviderEmit<SwayResponse>(
      mergedConfig,
      async ({ configHash, result }) => {
        if ('error' in result) {
          queue.error(result.error);
        } else {
          queue.output(await createState(result.output, configHash));
        }
      },
    );
  });
}