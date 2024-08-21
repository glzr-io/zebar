import { Window as TauriWindow } from '@tauri-apps/api/window';

import type { WindowConfig, WindowZOrder } from '~/user-config';

export interface ZebarContext<TProviders = {}> {
  /**
   * Parsed window config.
   */
  config: WindowConfig;

  /**
   * Map of this element's providers and their variables.
   */
  providers: TProviders;

  currentWindow: ZebarWindow;

  allWindows: ZebarWindow;

  currentMonitor: Monitor;

  allMonitors: Monitor;

  /**
   * Opens a new window by a relative path to its config file.
   */
  openWindow: (
    configPath: string,
    args?: Record<string, string>,
  ) => Promise<void>;
}

export interface ZebarWindow {
  tauri: TauriWindow;
  setZOrder: (zOrder: WindowZOrder) => Promise<void>;
}

export interface Monitor {
  /** Human-readable name of the monitor */
  name: string | null;

  /** Width of monitor in physical pixels. */
  width: number;

  /** Height of monitor in physical pixels. */
  height: number;

  /** X-coordinate of monitor in physical pixels. */
  x: number;

  /** Y-coordinate of monitor in physical pixels. */
  y: number;

  /** Scale factor to map physical pixels to logical pixels. */
  scaleFactor: number;
}
