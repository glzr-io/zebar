import { createResource } from 'solid-js';

import { getMonitorPosition } from '~/desktop';

// TODO: Remove this in favour of actually reading env/args on startup.
export function getConfigVariables() {
  const [configVariables] = createResource(async () => {
    const monitorPosition = await getMonitorPosition();

    return {
      screen_x: monitorPosition.x.toString(),
      screen_y: monitorPosition.y.toString(),
      screen_width: monitorPosition.width.toString(),
      screen_height: monitorPosition.height.toString(),
    };
  });

  return configVariables;
}
