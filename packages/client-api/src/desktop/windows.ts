import { getCurrentWindow } from '@tauri-apps/api/window';

import { desktopCommands } from './desktop-commands';

export type ZOrder = 'bottom_most' | 'top_most' | 'normal';

export interface Window {
  /**
   * The underlying Tauri window.
   */
  readonly tauri: ReturnType<typeof getCurrentWindow>;

  /**
   * Sets the z-order of the Tauri window.
   */
  setZOrder(zOrder: ZOrder): Promise<void>;
}

/**
 * Gets the window of the current widget.
 */
export function currentWindow(): Window {
  return {
    get tauri() {
      return getCurrentWindow();
    },
    setZOrder,
  };
}

async function setZOrder(zOrder: ZOrder) {
  if (zOrder === 'bottom_most') {
    await getCurrentWindow().setAlwaysOnBottom(true);
  } else if (zOrder === 'top_most') {
    await desktopCommands.setAlwaysOnTop();
  } else {
    await getCurrentWindow().setAlwaysOnTop(false);
  }
}
