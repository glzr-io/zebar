import { createStore } from 'solid-js/store';

import { MonitorInfo, WindowInfo } from '~/desktop';
import { SelfProviderConfig } from '~/user-config';
import { memoize } from '~/utils';

export interface SelfVariables {
  args: Record<string, string>;
  env: Record<string, string>;
  currentWindow: WindowInfo;
  currentMonitor?: MonitorInfo;
}

export const createSelfProvider = memoize((_: SelfProviderConfig) => {
  const [selfVariables] = createStore<SelfVariables>(getVariables());

  function getVariables() {
    const { args, env, currentWindow, currentMonitor } =
      window.__ZEBAR_INIT_STATE;
    return { args, env, currentWindow, currentMonitor };
  }

  return {
    get args() {
      return selfVariables.args;
    },
    get env() {
      return selfVariables.env;
    },
    get currentWindow() {
      return selfVariables.currentWindow;
    },
    get currentMonitor() {
      return selfVariables.currentMonitor;
    },
  };
});
