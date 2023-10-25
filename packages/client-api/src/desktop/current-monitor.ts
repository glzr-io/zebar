import { currentMonitor as getCurrentMonitor } from '@tauri-apps/api/window';

export async function getMonitorPosition() {
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
