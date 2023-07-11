import { invoke as tauriInvoke, InvokeArgs } from '@tauri-apps/api/tauri';

import { useLogger } from '../logging';
import { memoize } from '../utils';

function isTauri(): boolean {
  return !!(window && window.__TAURI__);
}

export const useDesktopCommands = memoize(() => {
  const logger = useLogger('useDesktopCommands');

  async function invoke<T>(command: string, args?: InvokeArgs): Promise<T> {
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

  function readConfigFile(): Promise<string> {
    return invoke<string>('read_config_file');
  }

  // TODO: Implement this. Should kill the window and show error dialog. If
  // there are no windows remaining, then exit the app.
  function exitWithError(message: string): Promise<void> {
    throw new Error(message);
  }

  return {
    readConfigFile,
    exitWithError,
  };
});
