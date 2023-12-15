import {
  Monitor,
  availableMonitors as getAvailableMonitors,
  currentMonitor as getCurrentMonitor,
  primaryMonitor as getPrimaryMonitor,
} from '@tauri-apps/api/window';

import { MonitorInfo } from './shared/monitor-info.model';

let fetchMonitorsPromise: ReturnType<typeof fetchMonitors> | null = null;

export async function getMonitors() {
  return fetchMonitorsPromise ?? (fetchMonitorsPromise = fetchMonitors());
}

async function fetchMonitors() {
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

  return {
    currentMonitor: currentMonitor ? toMonitorInfo(currentMonitor) : null,
    primaryMonitor: primaryMonitor ? toMonitorInfo(primaryMonitor) : null,
    secondaryMonitors: secondaryMonitors.map(toMonitorInfo),
    allMonitors: allMonitors.map(toMonitorInfo),
  };
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
