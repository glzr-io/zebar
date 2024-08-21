import { getCurrentWindow } from '@tauri-apps/api/window';
import { getOwner } from 'solid-js';

import { getOpenWindowArgs, openWindow, showErrorDialog } from './desktop';
import { createLogger } from '~/utils';
import type { ZebarContext } from './zebar-context.model';

const logger = createLogger('init-window');

/**
 * Handles initialization.
 */
export function init(callback: (context: ZebarContext) => void) {
  initAsync().then(callback);
}

/**
 * Handles initialization.
 */
export async function initAsync(): Promise<ZebarContext> {
  try {
    // TODO: Create new root if owner is null.
    const owner = getOwner()!;

    const windowState =
      window.__ZEBAR_OPEN_ARGS ??
      (await getOpenWindowArgs(getCurrentWindow().label));

    return {
      config: windowState.config,
      providers: windowState.providers,
      openWindow,
      // @ts-ignore - TODO
      currentWindow: {},
      // @ts-ignore - TODO
      allWindows: [],
      // @ts-ignore - TODO
      currentMonitor: {},
      // @ts-ignore - TODO
      allMonitors: [],
    };
  } catch (err) {
    logger.error('Failed to initialize window:', err);

    await showErrorDialog({
      title: 'Failed to initialize window',
      error: err,
    });

    // Error during window initialization is unrecoverable, so we close
    // the window.
    getCurrentWindow().close();
    throw err;
  }
}
