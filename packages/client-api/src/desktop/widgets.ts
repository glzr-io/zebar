import { join } from '@tauri-apps/api/path';

import { desktopCommands } from './desktop-commands';
import type { WidgetPlacement } from '~/config';
import { currentWindow, type WidgetWindow } from './windows';

export interface Widget {
  /**
   * Unique identifier for the widget instance.
   */
  id: string;

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
    configPath: state.configPath,
    htmlPath: state.htmlPath,
    window: currentWindow(),
  };
}

/**
 * Opens a widget by its config path and chosen placement.
 *
 * Config path is relative within the Zebar config directory.
 */
export async function startWidget(
  configPath: string,
  placement: WidgetPlacement,
) {
  // Ensure the config path ends with '.zebar.json'.
  const filePath = configPath.endsWith('.zebar.json')
    ? configPath
    : `${configPath}.zebar.json`;

  const absolutePath = await join(
    getWidgetState().configPath,
    '../',
    filePath,
  );

  return desktopCommands.startWidget(absolutePath, placement);
}

/**
 * Opens a widget by its config path and a preset name.
 *
 * Config path is relative within the Zebar config directory.
 */
export async function startWidgetPreset(
  configPath: string,
  presetName: string,
) {
  // Ensure the config path ends with '.zebar.json'.
  const filePath = configPath.endsWith('.zebar.json')
    ? configPath
    : `${configPath}.zebar.json`;

  const absolutePath = await join(
    getWidgetState().configPath,
    '../',
    filePath,
  );

  return desktopCommands.startPreset(absolutePath, presetName);
}
