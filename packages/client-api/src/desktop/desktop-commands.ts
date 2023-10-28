import { invoke as tauriInvoke, InvokeArgs } from '@tauri-apps/api/tauri';

import { createLogger } from '../utils';

const logger = createLogger('desktop-commands');

/**
 * Reads config file from disk. Creates file if it doesn't exist.
 */
export function readConfigFile(): Promise<string> {
  return invoke<string>('read_config_file');
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

  if (!isTauri()) {
    throw new Error('Cannot invoke a command without attaching to Tauri.');
  }

  try {
    const response = await tauriInvoke<T>(command, args);
    logger.info(`Response for calling '${command}':`, response);

    return response;
  } catch (err) {
    throw new Error(`Command '${command}' failed: ${err}`);
  }
}

/**
 * Whether the current window is Tauri-enabled.
 */
export function isTauri(): boolean {
  return !!(window && window.__TAURI__);
}
