import {
  LogicalPosition,
  LogicalSize,
  getCurrent as getCurrentWindow,
  type Window,
} from '@tauri-apps/api/window';

import type { ZOrder } from '~/user-config';
import { createLogger } from '~/utils';
import { setAlwaysOnTop } from './desktop-commands';

export interface WindowPosition {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface WindowStyles {
  zOrder: ZOrder;
  showInTaskbar: boolean;
  resizable: boolean;
}

const logger = createLogger('current-window');

export async function setWindowPosition(
  position: Partial<WindowPosition>,
) {
  logger.debug(`Setting window position to:`, position);

  const window = await getCurrentWindow();

  // Avoid setting position if neither x/y are defined.
  if (position.x !== undefined || position.y !== undefined) {
    await window.setPosition(
      new LogicalPosition(
        position.x ?? (await window.outerPosition()).x,
        position.y ?? (await window.outerPosition()).y,
      ),
    );
  }

  // Avoid setting size if neither width/height are defined.
  if (position.width !== undefined || position.height !== undefined) {
    await window.setSize(
      new LogicalSize(
        position.width ?? (await window.outerSize()).width,
        position.height ?? (await window.outerSize()).height,
      ),
    );
  }
}

export async function setWindowStyles(styles: Partial<WindowStyles>) {
  const window = await getCurrentWindow();

  await Promise.all([
    window.setSkipTaskbar(!styles.showInTaskbar ?? false),
    window.setResizable(styles.resizable ?? false),
    setWindowZOrder(window, styles.zOrder),
  ]);
}

async function setWindowZOrder(window: Window, zOrder?: ZOrder) {
  if (zOrder === 'always_on_bottom') {
    await window.setAlwaysOnBottom(true);
  } else if (zOrder === 'always_on_top') {
    await setAlwaysOnTop();
  } else {
    await window.setAlwaysOnTop(false);
  }
}
