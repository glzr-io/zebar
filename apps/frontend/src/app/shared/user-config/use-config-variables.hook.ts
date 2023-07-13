import { currentMonitor as getCurrentMonitor } from '@tauri-apps/api/window';

import { memoize } from '../utils';
import { createResource } from 'solid-js';
import { useDesktopCommands } from '../desktop';

export const useConfigVariables = memoize(() => {
  const commands = useDesktopCommands();

  const [currentMonitor] = createResource(() => {
    const currentMonitor = getCurrentMonitor();

    if (!currentMonitor) {
      commands.exitWithError('Unable to detect current monitor.');
    }

    return currentMonitor;
  });

  const [configVariables] = createResource(currentMonitor, currentMonitor => {
    return {
      screen_x: currentMonitor.position.x,
      screen_y: currentMonitor.position.y,
      screen_width: currentMonitor.size.width,
      screen_height: currentMonitor.size.height,
    };
  });

  return configVariables;
});
