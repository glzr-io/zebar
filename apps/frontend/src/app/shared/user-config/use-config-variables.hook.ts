import { createResource } from 'solid-js';

import { useCurrentMonitor } from '../desktop';
import { memoize } from '../utils';

export const useConfigVariables = memoize(() => {
  const currentMonitor = useCurrentMonitor();

  const [configVariables] = createResource(async () => {
    const monitorPosition = await currentMonitor.getPosition();

    return {
      screen_x: monitorPosition.x.toString(),
      screen_y: monitorPosition.y.toString(),
      screen_width: monitorPosition.width.toString(),
      screen_height: monitorPosition.height.toString(),
    };
  });

  return configVariables;
});
