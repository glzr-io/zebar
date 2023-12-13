import { currentMonitor as getCurrentMonitor } from '@tauri-apps/api/window';
import { createResource } from 'solid-js';

// TODO: Should probably be changed to `useMonitors`.
export async function useCurrentMonitor() {
  const currentMonitor = createResource(async () => {
    const monitor = await getCurrentMonitor();

    if (!monitor) {
      throw new Error('Unable to detect current monitor.');
    }

    return {
      x: monitor.position.x,
      y: monitor.position.y,
      width: monitor.size.width,
      height: monitor.size.height,
    };
  });

  // TODO: On display setting changes, refetch.

  return currentMonitor;
}
