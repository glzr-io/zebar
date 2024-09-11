import { getCurrentWindow } from '@tauri-apps/api/window';
import { join } from '@tauri-apps/api/path';

import {
  getWindowState,
  openWindow,
  setWindowZOrder,
  showErrorDialog,
} from './desktop';
import { createLogger } from '~/utils';
import type { ZebarContext } from './zebar-context.model';
import { createProvider } from './providers';

const logger = createLogger('init-window');

/**
 * Handles initialization.
 */
export async function init(): Promise<ZebarContext> {
  try {
    const currentWindow = getCurrentWindow();

    const windowState =
      window.__ZEBAR_INITIAL_STATE ??
      (await getWindowState(currentWindow.label));

    // @ts-ignore - TODO
    return {
      openWindow: async (configPath: string) => {
        const absolutePath = await join(
          windowState.configPath,
          '../',
          configPath,
        );

        return openWindow(absolutePath);
      },
      createProvider,
      currentWindow: {
        ...windowState,
        tauri: currentWindow,
        setZOrder: zOrder => {
          return setWindowZOrder(currentWindow, zOrder);
        },
      },
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
}
