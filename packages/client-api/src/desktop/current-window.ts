import {
  PhysicalPosition,
  PhysicalSize,
  getCurrentWindow,
  type Window,
} from '@tauri-apps/api/window';

import type { ZOrder } from '~/user-config';
import { createLogger } from '~/utils';
import { setAlwaysOnTop, setSkipTaskbar } from './desktop-commands';

export interface WindowPosition {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface WindowStyles {
  zOrder: ZOrder;
  shownInTaskbar: boolean;
  resizable: boolean;
}

const logger = createLogger('current-window');

export async function setWindowPosition(
  position: Partial<WindowPosition>,
) {
  logger.debug(`Setting window position to:`, position);

  const window = getCurrentWindow();

  // Avoid setting position if neither x/y are defined.
  if (position.x !== undefined || position.y !== undefined) {
    const newPosition = new PhysicalPosition(
      position.x ?? (await window.outerPosition()).x,
      position.y ?? (await window.outerPosition()).y,
    );

    // Set position twice to handle DPI changes on cross-monitor moves.
    await window.setPosition(newPosition);
    await window.setPosition(newPosition);
  }

  // Avoid setting size if neither width/height are defined.
  if (position.width !== undefined || position.height !== undefined) {
    const newSize = new PhysicalSize(
      position.width ?? (await window.outerSize()).width,
      position.height ?? (await window.outerSize()).height,
    );

    // Set size twice to handle DPI changes on cross-monitor moves.
    await window.setSize(newSize);
    await window.setSize(newSize);
  }
}

export async function setWindowStyles(styles: Partial<WindowStyles>) {
  const window = getCurrentWindow();

  await Promise.all([
    setSkipTaskbar(styles.shownInTaskbar !== true),
    window.setResizable(styles.resizable === true),
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
