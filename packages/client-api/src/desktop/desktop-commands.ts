import { InvokeArgs, invoke as tauriInvoke } from '@tauri-apps/api/tauri';

import { createLogger } from '../utils';
import { ProviderOptions, ProviderType } from '~/user-config';

const logger = createLogger('desktop-commands');

/**
 * Reads config file from disk. Creates file if it doesn't exist.
 */
export function readConfigFile(): Promise<string> {
  return invoke<string>('read_config_file');
}

// TODO: Add support for only fetching tracked data.
export function listenProvider(args: {
  type: ProviderType;
  optionsHash: string;
  options: ProviderOptions;
  trackedData: string[];
}): Promise<string> {
  return invoke<string>('listen_provider', args);
}

export function unlistenProvider(configHash: string): Promise<string> {
  return invoke<string>('unlisten_provider');
}

export function test(): Promise<string> {
  return invoke<string>('test');
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
