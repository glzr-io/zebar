import { getCurrentWindow } from '@tauri-apps/api/window';

import { getWindowState } from './desktop-commands';

let promise: Promise<any> | null = null;

export async function _getOpenWindowArgs() {
  return promise ?? (promise = fetchOpenWindowArgs());
}

async function fetchOpenWindowArgs() {
  if (window.__ZEBAR_INITIAL_STATE) {
    return window.__ZEBAR_INITIAL_STATE;
  }

  return getWindowState(getCurrentWindow().label);
}
