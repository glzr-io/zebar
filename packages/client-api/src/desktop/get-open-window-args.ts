import { getCurrentWindow } from '@tauri-apps/api/window';
import { createStore } from 'solid-js/store';

import type { OpenWindowArgs } from './shared';
import { getInitialState } from './desktop-commands';

const [openWindowArgs, setOpenWindowArgs] = createStore({
  value: null as OpenWindowArgs | null,
});

let promise: Promise<any> | null = null;

export async function _getOpenWindowArgs() {
  return promise ?? (promise = fetchOpenWindowArgs());
}

async function fetchOpenWindowArgs() {
  if (window.__ZEBAR_INITIAL_STATE) {
    return window.__ZEBAR_INITIAL_STATE;
  }

  return getInitialState(await getCurrentWindow().label);
}
