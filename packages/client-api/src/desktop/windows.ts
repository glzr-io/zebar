import { getCurrentWindow } from '@tauri-apps/api/window';

import { desktopCommands } from './desktop-commands';

export type ZOrder = 'always_on_bottom' | 'always_on_top' | 'normal';

export interface Window {
  /**
   * The underlying Tauri window.
   */
  readonly tauri: ReturnType<typeof getCurrentWindow>;

  /**
   * Sets the z-order of the window.
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
  if (zOrder === 'always_on_bottom') {
    await getCurrentWindow().setAlwaysOnBottom(true);
  } else if (zOrder === 'always_on_top') {
    await desktopCommands.setAlwaysOnTop();
  } else {
    await getCurrentWindow().setAlwaysOnTop(false);
  }
}
