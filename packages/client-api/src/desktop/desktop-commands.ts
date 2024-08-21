import {
  type InvokeArgs,
  invoke as tauriInvoke,
} from '@tauri-apps/api/core';

import { createLogger } from '../utils';
import type { ProviderConfig } from '~/user-config';
import type { OpenWindowArgs } from './shared';

const logger = createLogger('desktop-commands');

/**
 * Reads config file from disk. Creates file if it doesn't exist.
 */
export function readConfigFile(): Promise<string> {
  return invoke<string>('read_config_file');
}

/**
 * Get args used to open the window with the {@link windowLabel}.
 */
export function getOpenWindowArgs(
  windowLabel: string,
): Promise<OpenWindowArgs | null> {
  return invoke<OpenWindowArgs | null>('get_open_window_args', {
    windowLabel,
  });
}

export function openWindow(
  windowId: string,
  args: Record<string, string> = {},
): Promise<void> {
  return invoke<void>('open_window', { windowId, args });
}

// TODO: Add support for only fetching selected variables.
export function listenProvider(args: {
  configHash: string;
  config: ProviderConfig;
  trackedAccess: string[];
}): Promise<void> {
  return invoke<void>('listen_provider', args);
}

export function unlistenProvider(configHash: string): Promise<void> {
  return invoke<void>('unlisten_provider', { configHash });
}

export function setAlwaysOnTop(): Promise<void> {
  return invoke<void>('set_always_on_top');
}

export function setSkipTaskbar(skip: boolean): Promise<void> {
  return invoke<void>('set_skip_taskbar', { skip });
}

// TODO: Implement this. Should kill the window and show error dialog. If
// there are no windows remaining, then exit the app.
export function exitWithError(message: string): never {
  throw new Error(message);
}

/**
 * Invoke a Tauri command with logging and error handling.
 */
export async function invoke<T>(
  command: string,
  args?: InvokeArgs,
): Promise<T> {
  logger.info(`Calling '${command}' with args:`, args ?? {});

  try {
    const response = await tauriInvoke<T>(command, args);
    logger.info(`Response for calling '${command}':`, response);

    return response;
  } catch (err) {
    throw new Error(`Command '${command}' failed: ${err}`);
  }
}
