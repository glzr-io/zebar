import { join } from '@tauri-apps/api/path';

import { desktopCommands } from './desktop-commands';

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
  };
}

/**
 * Opens a new widget instance by a relative path to its config file.
 */
export async function startWidget(configPath: string) {
  // Ensure the config path ends with '.zebar.json'.
  const filePath = configPath.endsWith('.zebar.json')
    ? configPath
    : `${configPath}.zebar.json`;

  const absolutePath = await join(
    getWidgetState().configPath,
    '../',
    filePath,
  );

  return desktopCommands.startWidget(absolutePath);
}
