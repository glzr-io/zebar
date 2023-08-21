import { currentMonitor as getCurrentMonitor } from '@tauri-apps/api/window';

import { memoize } from '../utils';

/**
 * Hook for interacting with Tauri's monitor-related APIs.
 */
export const useCurrentMonitor = memoize(() => {
  async function getPosition() {
    const currentMonitor = await getCurrentMonitor();

    if (!currentMonitor) {
      throw new Error('Unable to detect current monitor.');
    }

    return {
      x: currentMonitor.position.x,
      y: currentMonitor.position.y,
      width: currentMonitor.size.width,
      height: currentMonitor.size.height,
    };
  }

  return {
    getPosition,
  };
});
