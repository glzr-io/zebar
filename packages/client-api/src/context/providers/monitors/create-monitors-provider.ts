import { createStore } from 'solid-js/store';

import { MonitorInfo } from '~/desktop';
import { MonitorsProviderConfig } from '~/user-config';
import { memoize } from '~/utils';

export interface MonitorsVariables {
  primary?: MonitorInfo;
  secondary: MonitorInfo[];
  all: MonitorInfo[];
}

export const createMonitorsProvider = memoize((_: MonitorsProviderConfig) => {
  const [monitorVariables] = createStore<MonitorsVariables>(getVariables());

  function getVariables() {
    const { primaryMonitor, monitors } = window.__ZEBAR_INIT_STATE;

    const secondaryMonitors = monitors.filter(
      monitor => !primaryMonitor || isMatch(monitor, primaryMonitor),
    );

    return {
      primary: primaryMonitor,
      secondary: secondaryMonitors,
      all: monitors,
    };
  }

  function isMatch(monitorA: MonitorInfo, monitorB: MonitorInfo) {
    return (
      monitorA.name === monitorB.name &&
      monitorA.x === monitorB.x &&
      monitorA.y === monitorB.y &&
      monitorA.width === monitorB.width &&
      monitorA.height === monitorB.height
    );
  }

  return {
    get primary() {
      return monitorVariables.primary;
    },
    get secondary() {
      return monitorVariables.secondary;
    },
    get all() {
      return monitorVariables.all;
    },
  };
});
