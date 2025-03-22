import { desktopCommands } from './desktop-commands';
import type { WidgetPlacement } from '~/config';
import { currentWindow, type WidgetWindow } from './windows';

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
   */
  window: WidgetWindow;

  /**
   * Whether the widget is in preview mode.
   *
   * Widgets get marked as previews if they are opened from another
   * preview widget.
   */
  isPreview: boolean;
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

  return {
    id: state.id,
    name: state.name,
    packId: state.packId,
    configPath: state.configPath,
    htmlPath: state.htmlPath,
    window: currentWindow(),
    isPreview: state.isPreview,
  };
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
  args: StartWidgetArgs,
) {
  return desktopCommands.startWidget(
    args.packId ?? currentWidget().packId,
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
