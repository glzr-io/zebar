import {
  Monitor,
  availableMonitors as getAvailableMonitors,
  currentMonitor as getCurrentMonitor,
  primaryMonitor as getPrimaryMonitor,
} from '@tauri-apps/api/window';
import { createStore } from 'solid-js/store';
import { Owner, createEffect, runWithOwner } from 'solid-js';

import { MonitorInfo } from './shared';

let fetchMonitorsPromise: ReturnType<typeof fetchMonitors> | null = null;

/**
 * Store of available monitors. Lazily initialize as values are fetched.
 */
const [monitorCache, setMonitorCache] = createStore({
  current: {
    value: null as MonitorInfo | null,
    isFetching: false,
    isInitialized: false,
  },
  primary: {
    value: null as MonitorInfo | null,
    isFetching: false,
    isInitialized: false,
  },
  secondary: {
    value: [] as MonitorInfo[],
    isFetching: false,
    isInitialized: false,
  },
  all: {
    value: [] as MonitorInfo[],
    isFetching: false,
    isInitialized: false,
  },
});

export async function getMonitors() {
  return fetchMonitorsPromise ?? (fetchMonitorsPromise = fetchMonitors());
}

// export async function _getAllMonitors(owner: Owner) {
//   return createSharedCache('all-monitors', async () => {
//     getAvailableMonitors(),
//   });
// }

// export async function __getPrimaryMonitor(owner: Owner) {
//   return createSharedCache('primary-monitor', async () => {
//     getPrimaryMonitor(),
//   });
// }

// export async function __getCurrentMonitor(owner: Owner) {
//   const monitorMap = await createSharedCache('monitor-map', async () =>
//     getPrimaryMonitor(),
//   );
// }

// export async function getMonitorCache(owner: Owner) {
//   const monitorCache = await createSharedCache('monitors', async () =>
//     getMonitorCache(),
//   );
// }

export async function _getPrimaryMonitor(owner: Owner) {
  if (monitorCache.primary.isInitialized) {
    return monitorCache.primary.value;
  }

  if (monitorCache.primary.isFetching) {
    runWithOwner(owner, () => {
      createEffect(() => {
        if (!monitorCache.primary.isFetching) {
          return Promise.resolve(monitorCache.primary.value);
        }
      });
    });
  }

  setMonitorCache('primary', 'isFetching', true);

  const primaryMonitor = await getPrimaryMonitor();
  const value = primaryMonitor ? toMonitorInfo(primaryMonitor) : null;

  setMonitorCache('primary', {
    value,
    isFetching: false,
    isInitialized: true,
  });

  return value;
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
