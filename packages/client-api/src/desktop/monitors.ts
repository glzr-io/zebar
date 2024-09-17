import {
  type Monitor,
  availableMonitors as getAvailableMonitors,
  currentMonitor as getCurrentMonitor,
  primaryMonitor as getPrimaryMonitor,
  getCurrentWindow,
} from '@tauri-apps/api/window';

import type { MonitorInfo } from './shared';

let createCachePromise: Promise<MonitorCache> | null = null;

interface MonitorCache {
  currentMonitor: MonitorInfo | null;
  primaryMonitor: MonitorInfo | null;
  secondaryMonitors: MonitorInfo[];
  allMonitors: MonitorInfo[];
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
    currentMonitor: currentMonitor ? toMonitorInfo(currentMonitor) : null,
    primaryMonitor: primaryMonitor ? toMonitorInfo(primaryMonitor) : null,
    secondaryMonitors: secondaryMonitors.map(toMonitorInfo),
    allMonitors: allMonitors.map(toMonitorInfo),
  };

  getCurrentWindow().onResized(() => updateCurrentMonitor());
  getCurrentWindow().onMoved(() => updateCurrentMonitor());

  // Update the current monitor when the window is moved or resized.
  async function updateCurrentMonitor() {
    const currentMonitor = await getCurrentMonitor();

    // TODO: Avoid mutating the cache object.
    Object.assign(monitorCache, {
      currentMonitor: currentMonitor
        ? toMonitorInfo(currentMonitor)
        : null,
    });
  }

  return monitorCache;
}

function isMatch(monitorA: Monitor, monitorB: Monitor) {
  return (
    monitorA.name === monitorB.name &&
    monitorA.position.x === monitorB.position.x &&
    monitorA.position.y === monitorB.position.y &&
    monitorA.size.width === monitorB.size.width &&
    monitorA.size.height === monitorB.size.height
  );
}

function toMonitorInfo(monitor: Monitor): MonitorInfo {
  return {
    name: monitor.name ?? '',
    scaleFactor: monitor.scaleFactor,
    width: monitor.size.width,
    height: monitor.size.height,
    x: monitor.position.x,
    y: monitor.position.y,
  };
}
