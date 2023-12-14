import { Owner } from 'solid-js';
import { createStore } from 'solid-js/store';

import { MonitorInfo, WindowInfo } from '~/desktop';
import { SelfProviderConfig } from '~/user-config';

export interface SelfVariables {
  args: Record<string, string>;
  env: Record<string, string>;
  currentWindow: WindowInfo;
  currentMonitor?: MonitorInfo;
}

export async function createSelfProvider(_: SelfProviderConfig, __: Owner) {
  const [selfVariables] = createStore<SelfVariables>(getVariables());

  function getVariables() {
    // TODO: Handle fetching initial state if state on window is not defined.
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
}
