import {
  type Monitor as TauriMonitor,
  availableMonitors as getAvailableMonitors,
  currentMonitor as getCurrentMonitor,
  primaryMonitor as getPrimaryMonitor,
  getCurrentWindow,
} from '@tauri-apps/api/window';

export interface Monitor {
  /**
   * Human-readable name of the monitor.
   */
  name: string | null;

  /**
   * Width of monitor in physical pixels.
   */
  width: number;

  /**
   * Height of monitor in physical pixels.
   */
  height: number;

  /**
   * X-coordinate of monitor in physical pixels.
   */
  x: number;

  /**
   * Y-coordinate of monitor in physical pixels.
   */
  y: number;

  /**
   * Scale factor to map physical pixels to logical pixels.
   */
  scaleFactor: number;
}

let createCachePromise: Promise<MonitorCache> | null = null;

interface MonitorCache {
  currentMonitor: Monitor | null;
  primaryMonitor: Monitor | null;
  secondaryMonitors: Monitor[];
  allMonitors: Monitor[];
}

export async function getMonitors() {
  return createCachePromise ?? (createCachePromise = createMonitorCache());
}

async function createMonitorCache() {
  const [currentMonitor, primaryMonitor, allMonitors] = await Promise.all([
    getCurrentMonitor(),
    getPrimaryMonitor(),
    getAvailableMonitors(),
  ]);

  const secondaryMonitors = allMonitors.filter(
    monitor => !primaryMonitor || !isMatch(monitor, primaryMonitor),
  );

  // TODO: Refetch on display setting changes. Create a store with the current
  // return value, and refresh it in an effect when displays are changed.
  // Ref https://github.com/tauri-apps/tauri/issues/8405

  const monitorCache = {
    currentMonitor: currentMonitor ? toMonitor(currentMonitor) : null,
    primaryMonitor: primaryMonitor ? toMonitor(primaryMonitor) : null,
    secondaryMonitors: secondaryMonitors.map(toMonitor),
    allMonitors: allMonitors.map(toMonitor),
  };

  getCurrentWindow().onResized(() => updateCurrentMonitor());
  getCurrentWindow().onMoved(() => updateCurrentMonitor());

  // Update the current monitor when the window is moved or resized.
  async function updateCurrentMonitor() {
    const currentMonitor = await getCurrentMonitor();

    // TODO: Avoid mutating the cache object.
    Object.assign(monitorCache, {
      currentMonitor: currentMonitor ? toMonitor(currentMonitor) : null,
    });
  }

  return monitorCache;
}

function isMatch(monitorA: TauriMonitor, monitorB: TauriMonitor) {
  return (
    monitorA.name === monitorB.name &&
    monitorA.position.x === monitorB.position.x &&
    monitorA.position.y === monitorB.position.y &&
    monitorA.size.width === monitorB.size.width &&
    monitorA.size.height === monitorB.size.height
  );
}

function toMonitor(monitor: TauriMonitor): Monitor {
  return {
    name: monitor.name,
    width: monitor.size.width,
    height: monitor.size.height,
    x: monitor.position.x,
    y: monitor.position.y,
    scaleFactor: monitor.scaleFactor,
  };
}
