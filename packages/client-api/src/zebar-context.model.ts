import { Window as TauriWindow } from '@tauri-apps/api/window';

import type { WidgetConfig, ZOrder } from '~/user-config';
import type {
  ProviderConfig,
  ProviderGroup,
  ProviderGroupConfig,
  ProviderMap,
} from './providers';

export interface ZebarContext {
  currentWidget: Widget;

  /**
   * Opens a new widget instance by a relative path to its config file.
   */
  startWidget(configPath: string): Promise<void>;

  /**
   * Creates a provider, which is a collection of functions and variables
   * that can change over time. Alternatively, multiple providers can be
   * created using {@link createProviderGroup}.
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
   * Creates multiple providers at once. A provider is a collection of
   * functions and variables that can change over time. Alternatively, a
   * single provider can be created using {@link createProvider}.
   */
  createProviderGroup<T extends ProviderGroupConfig>(
    configMap: T,
  ): Promise<ProviderGroup<T>>;
}

export interface Widget {
  /**
   * Unique identifier for the widget instance.
   */
  instanceId: string;

  /**
   * Parsed config of the widget instance.
   */
  config: WidgetConfig;

  /**
   * Absolute path to the widget config file.
   */
  configPath: string;

  /**
   * Underlying Tauri window.
   */
  tauri: TauriWindow;

  /**
   * Sets the z-order of the underlying window.
   */
  setZOrder(zOrder: ZOrder): Promise<void>;
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
