import { Suspense } from 'solid-js';
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
  const [monitorVariables] = createStore<MonitorsVariables>(
    getInitialVariables(),
  );

  function getInitialVariables() {
    const { primaryMonitor, allMonitors } = window.__ZEBAR_INIT_STATE;

    const secondaryMonitors = allMonitors.filter(
      monitor => monitor.name !== primaryMonitor.name,
    );

    return {
      primary: primaryMonitor,
      secondary: secondaryMonitors,
      all: allMonitors,
    };
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
