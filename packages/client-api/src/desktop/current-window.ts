import {
  PhysicalPosition,
  PhysicalSize,
  getCurrent as getCurrentWindow,
} from '@tauri-apps/api/window';

import { createLogger } from '~/utils';
import { getMonitorPosition } from './current-monitor';

export interface WindowPosition {
  x?: number;
  y?: number;
  width?: number;
  height?: number;
}

export interface WindowStyles {
  alwaysOnTop?: boolean;
  showInTaskbar?: boolean;
  resizable?: boolean;
}

const logger = createLogger('current-window');

export async function setWindowPosition(position: WindowPosition) {
  const monitorPosition = await getMonitorPosition();

  const parsedPosition = {
    x: position.x ? position.x : monitorPosition.x,
    y: position.y ? position.y : monitorPosition.y,
    width: position.width ? position.width : monitorPosition.width,
    height: position.height ? position.height : 30,
  };

  logger.debug(`Setting window position to:`, parsedPosition);

  await getCurrentWindow().setPosition(
    new PhysicalPosition(parsedPosition.x, parsedPosition.y),
  );

  await getCurrentWindow().setSize(
    new PhysicalSize(parsedPosition.width, parsedPosition.height),
  );
}

export async function setWindowStyles(styles: WindowStyles) {
  await getCurrentWindow().setAlwaysOnTop(styles.alwaysOnTop ?? true);
  await getCurrentWindow().setSkipTaskbar(!styles.showInTaskbar ?? false);
  await getCurrentWindow().setResizable(styles.resizable ?? false);
}
