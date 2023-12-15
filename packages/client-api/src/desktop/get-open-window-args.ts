import { getCurrent } from '@tauri-apps/api/window';
import { createStore } from 'solid-js/store';

import { OpenWindowArgs } from './shared';
import { getOpenWindowArgs } from './desktop-commands';

const [openWindowArgs, setOpenWindowArgs] = createStore({
  value: null as OpenWindowArgs | null,
});

let promise: Promise<any> | null = null;

export async function _getOpenWindowArgs() {
  return promise ?? (promise = fetchOpenWindowArgs());
}

async function fetchOpenWindowArgs() {
  if (window.__ZEBAR_OPEN_ARGS) {
    return window.__ZEBAR_OPEN_ARGS;
  }

  return getOpenWindowArgs(await getCurrent().label);
}
