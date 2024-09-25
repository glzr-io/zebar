import {
  type InvokeArgs,
  invoke as tauriInvoke,
} from '@tauri-apps/api/core';

import { createLogger } from '../utils';
import type { ProviderConfig } from '~/providers';

const logger = createLogger('desktop-commands');

export const desktopCommands = {
  startWidget,
  listenProvider,
  unlistenProvider,
  setAlwaysOnTop,
  setSkipTaskbar,
};

function startWidget(configPath: string): Promise<void> {
  return invoke<void>('start_widget', { configPath });
}

function listenProvider(args: {
  configHash: string;
  config: ProviderConfig;
}): Promise<void> {
  return invoke<void>('listen_provider', args);
}

function unlistenProvider(configHash: string): Promise<void> {
  return invoke<void>('unlisten_provider', { configHash });
}

function setAlwaysOnTop(): Promise<void> {
  return invoke<void>('set_always_on_top');
}

function setSkipTaskbar(skip: boolean): Promise<void> {
  return invoke<void>('set_skip_taskbar', { skip });
}

/**
 * Invoke a Tauri command with logging and error handling.
 */
async function invoke<T>(command: string, args?: InvokeArgs): Promise<T> {
  logger.info(`Calling '${command}' with args:`, args ?? {});

  try {
    const response = await tauriInvoke<T>(command, args);
    logger.info(`Response for calling '${command}':`, response);

    return response;
  } catch (err) {
    logger.error(`Command '${command}' failed: ${err}`);
    throw new Error(`Command '${command}' failed: ${err}`);
  }
}
