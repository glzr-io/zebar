import { Window as TauriWindow } from '@tauri-apps/api/window';

import type { WindowConfig, WindowZOrder } from '~/user-config';
import type {
  ProviderConfig,
  ProviderGroup,
  ProviderGroupConfig,
  ProviderMap,
} from './providers';

export interface ZebarContext {
  currentWindow: ZebarWindow;

  allWindows: ZebarWindow;

  currentMonitor: Monitor;

  allMonitors: Monitor;

  /**
   * Opens a new window by a relative path to its config file.
   */
  openWindow(
    configPath: string,
    args?: Record<string, string>,
  ): Promise<void>;

  /**
   * Creates an instance of a provider. Alternatively, multiple
   * providers can be created using {@link createProviderGroup}.
   *
   * Waits until the provider has emitted either its first output or first
   * error. The provider will continue to output until its `stop` function is
   * called.
   *
   * @throws If the provider config is invalid. *Does not throw* if the
   * provider's first emission is an error.
   */
  createProvider<T extends ProviderConfig>(
    providerConfig: T,
  ): Promise<ProviderMap[T['type']]>;

  /**
   * Creates multiple provider instances at once. Alternatively, a single
   * provider can be created using {@link createProvider}.
   */
  createProviderGroup<T extends ProviderGroupConfig>(
    configMap: T,
  ): Promise<ProviderGroup<T>>;
}

export interface ZebarWindow {
  /**
   * Unique identifier for the window.
   */
  windowId: string;

  /**
   * Parsed window config.
   */
  config: WindowConfig;

  /**
   * Absolute path to the window's config file.
   */
  configPath: string;

  /**
   * Tauri window instance.
   */
  tauri: TauriWindow;

  /**
   * Sets the z-order of the window.
   */
  setZOrder(zOrder: WindowZOrder): Promise<void>;
}

export interface Monitor {
  /**
   * Human-readable name of the monitor.
   */
  name: string | null;

  /**
   * Width of monitor in physical pixels.
   */
  width: number;

  /**
   * Height of monitor in physical pixels.
   */
  height: number;

  /**
   * X-coordinate of monitor in physical pixels.
   */
  x: number;

  /**
   * Y-coordinate of monitor in physical pixels.
   */
  y: number;

  /**
   * Scale factor to map physical pixels to logical pixels.
   */
  scaleFactor: number;
}
