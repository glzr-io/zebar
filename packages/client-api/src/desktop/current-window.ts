import {
  PhysicalPosition,
  PhysicalSize,
  getCurrent as getCurrentWindow,
} from '@tauri-apps/api/window';

import { ZOrder } from '~/user-config';
import { createLogger } from '~/utils';

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

  // TODO: Avoid setting position if neither x/y are defined.
  await window.setPosition(
    new PhysicalPosition(
      position.x ?? (await window.outerPosition()).x,
      position.y ?? (await window.outerPosition()).y,
    ),
  );

  // TODO: Avoid setting size if neither width/height are defined.
  await window.setSize(
    new PhysicalSize(
      position.width ?? (await window.outerSize()).width,
      position.height ?? (await window.outerSize()).height,
    ),
  );
}

export async function setWindowStyles(styles: Partial<WindowStyles>) {
  const window = await getCurrentWindow();

  window.setSkipTaskbar(!styles.showInTaskbar ?? false);
  window.setResizable(styles.resizable ?? false);

  if (styles.zOrder === 'always_on_bottom') {
    window.setAlwaysOnBottom(true);
  } else if (styles.zOrder === 'always_on_top') {
    window.setAlwaysOnTop(true);
  } else {
    window.setAlwaysOnTop(false);
  }
}
