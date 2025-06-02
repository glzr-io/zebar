import { getCurrentWindow } from '@tauri-apps/api/window';

import { desktopCommands } from './desktop-commands';
import type { WidgetPlacement } from '~/config';

export interface Widget {
  /**
   * Unique identifier for the widget instance.
   */
  id: string;

  /**
   * Name of the widget.
   */
  name: string;

  /**
   * Unique identifier for the widget pack.
   */
  packId: string;

  /**
   * Absolute path to the widget's config file.
   */
  configPath: string;

  /**
   * Absolute path to the widget's HTML file.
   */
  htmlPath: string;

  /**
   * The window of the widget.
   *
   * @deprecated Use {@link tauriWindow} and {@link setZOrder} instead
   * (e.g. `currentWidget().setZOrder('bottom_most')`).
   */
  window: {
    get tauri(): ReturnType<typeof getCurrentWindow>;
    setZOrder(zOrder: ZOrder): Promise<void>;
  };

  /**
   * The underlying Tauri window.
   */
  tauriWindow: ReturnType<typeof getCurrentWindow>;

  /**
   * Whether the widget is in preview mode.
   *
   * Widgets get marked as previews if they are opened from another
   * preview widget.
   */
  isPreview: boolean;

  /**
   * Sets the z-order of the widget's window.
   */
  setZOrder(zOrder: ZOrder): Promise<void>;

  /**
   * Closes the widget's window.
   *
   * Same as calling `tauriWindow.close()`.
   */
  close(): Promise<void>;
}

function getWidgetState(): Widget {
  if (window.__ZEBAR_STATE) {
    return window.__ZEBAR_STATE;
  }

  const widgetState = sessionStorage.getItem('ZEBAR_STATE');

  if (!widgetState) {
    throw new Error('No widget state found.');
  }

  return JSON.parse(widgetState);
}

export function currentWidget(): Widget {
  const state = getWidgetState();
  const tauriWindow = getCurrentWindow();

  return {
    id: state.id,
    name: state.name,
    packId: state.packId,
    configPath: state.configPath,
    htmlPath: state.htmlPath,
    window: {
      get tauri() {
        return tauriWindow;
      },
      setZOrder: (zOrder: ZOrder) => setZOrder(tauriWindow, zOrder),
    },
    tauriWindow,
    isPreview: state.isPreview,
    setZOrder: (zOrder: ZOrder) => setZOrder(tauriWindow, zOrder),
    close: () => close(tauriWindow),
  };
}

export type ZOrder = 'bottom_most' | 'top_most' | 'normal';

async function setZOrder(
  window: ReturnType<typeof getCurrentWindow>,
  zOrder: ZOrder,
) {
  if (zOrder === 'bottom_most') {
    await window.setAlwaysOnBottom(true);
  } else if (zOrder === 'top_most') {
    await desktopCommands.setAlwaysOnTop();
  } else {
    await window.setAlwaysOnTop(false);
  }
}

async function close(window: ReturnType<typeof getCurrentWindow>) {
  await window.close();
}

export interface StartWidgetArgs {
  packId?: string;
}

/**
 * Opens a widget by its name and chosen placement.
 */
export async function startWidget(
  widgetName: string,
  placement: WidgetPlacement,
  args?: StartWidgetArgs,
) {
  return desktopCommands.startWidget(
    args?.packId ?? currentWidget().packId,
    widgetName,
    placement,
    getWidgetState().isPreview,
  );
}

export interface StartWidgetPresetArgs {
  packId?: string;
}

/**
 * Opens a widget by its name and a preset name.
 */
export async function startWidgetPreset(
  widgetName: string,
  presetName: string,
  args?: StartWidgetPresetArgs,
) {
  return desktopCommands.startWidgetPreset(
    args?.packId ?? currentWidget().packId,
    widgetName,
    presetName,
    getWidgetState().isPreview,
  );
}
