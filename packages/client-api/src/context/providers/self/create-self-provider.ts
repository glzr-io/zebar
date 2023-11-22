import { createStore } from 'solid-js/store';

import { SelfProviderConfig } from '~/user-config';
import { memoize } from '~/utils';

export interface SelfVariables {
  currentWindow: WindowInfo;
  currentMonitor?: MonitorInfo;
  primaryMonitor?: MonitorInfo;
  monitors: MonitorInfo[];
}

export interface MonitorInfo {
  name: string;
  x: number;
  y: number;
  width: number;
  height: number;
  scaleFactor: number;
}

export interface WindowInfo {
  x: number;
  y: number;
  width: number;
  height: number;
  scaleFactor: number;
}

export const createSelfProvider = memoize((_: SelfProviderConfig) => {
  const [selfVariables] = createStore<SelfVariables>(window.__ZEBAR_INIT_STATE);

  return {
    get currentWindow() {
      return selfVariables.currentWindow;
    },
    get currentMonitor() {
      return selfVariables.currentMonitor;
    },
    get primaryMonitor() {
      return selfVariables.primaryMonitor;
    },
    get monitors() {
      return selfVariables.monitors;
    },
  };
});
