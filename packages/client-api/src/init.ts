import { getCurrentWindow } from '@tauri-apps/api/window';
import { createRoot, getOwner, runWithOwner } from 'solid-js';

import { getInitialState, openWindow, showErrorDialog } from './desktop';
import { createLogger } from '~/utils';
import type { ZebarContext } from './zebar-context.model';
import { createProvider } from './providers';

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
  return withReactiveContext(async () => {
    try {
      const currentWindow = getCurrentWindow();

      const initialState =
        window.__ZEBAR_INITIAL_STATE ??
        (await getInitialState(currentWindow.label));

      await currentWindow.show();

      // @ts-ignore - TODO
      return {
        // @ts-ignore - TODO
        config: initialState.config,
        openWindow,
        createProvider: config => {
          return createProvider(config, getOwner()!);
        },
        currentWindow: {},
        allWindows: [],
        currentMonitor: {},
        allMonitors: [],
      } as ZebarContext;
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
  });
}

/**
 * Runs callback in a reactive context (allows for SolidJS reactivity).
 */
function withReactiveContext<T>(callback: () => T) {
  const owner = getOwner();

  return owner
    ? (runWithOwner(owner, callback) as T)
    : createRoot(callback);
}
