import { Owner } from 'solid-js';
import { createStore } from 'solid-js/store';
import { getCurrent as getCurrentWindow } from '@tauri-apps/api/window';

import { getOpenWindowArgs } from '~/desktop';
import { SelfProviderConfig } from '~/user-config';

// TODO: Add window + monitor.
export interface SelfVariables {
  args: Record<string, string>;
  env: Record<string, string>;
}

export async function createSelfProvider(
  _: SelfProviderConfig,
  __: Owner,
) {
  const [selfVariables] = createStore<SelfVariables>(await getVariables());

  async function getVariables() {
    const { args, env } =
      window.__ZEBAR_OPEN_ARGS ??
      (await getOpenWindowArgs(await getCurrentWindow().label));

    return { args, env };
  }

  return {
    get args() {
      return selfVariables.args;
    },
    get env() {
      return selfVariables.env;
    },
  };
}
