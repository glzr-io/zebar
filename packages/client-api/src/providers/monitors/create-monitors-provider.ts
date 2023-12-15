import { Owner } from 'solid-js';

import { MonitorInfo, getMonitors } from '~/desktop';
import { MonitorsProviderConfig } from '~/user-config';

export interface MonitorsVariables {
  primary?: MonitorInfo;
  secondary: MonitorInfo[];
  all: MonitorInfo[];
}

export async function createMonitorsProvider(
  _: MonitorsProviderConfig,
  __: Owner,
) {
  const { primaryMonitor, secondaryMonitors, allMonitors } =
    await getMonitors();

  return {
    get primary() {
      return primaryMonitor;
    },
    get secondary() {
      return secondaryMonitors;
    },
    get all() {
      return allMonitors;
    },
  };
}
