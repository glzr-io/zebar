import {
  PhysicalPosition,
  PhysicalSize,
  getCurrent as getCurrentWindow,
} from '@tauri-apps/api/window';

import { createLogger } from '~/utils';

export interface WindowPosition {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface WindowStyles {
  alwaysOnTop: boolean;
  showInTaskbar: boolean;
  resizable: boolean;
}

const logger = createLogger('current-window');

export async function setWindowPosition(position: Partial<WindowPosition>) {
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
  window.setAlwaysOnTop(styles.alwaysOnTop ?? true);
  window.setSkipTaskbar(!styles.showInTaskbar ?? false);
  window.setResizable(styles.resizable ?? false);
}
