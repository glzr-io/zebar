import { invoke as tauriInvoke, InvokeArgs } from '@tauri-apps/api/tauri';

import { useLogger } from './logging/use-logger.hook';
import { memoize } from './utils/memoize';

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

  function readConfigFile(path: string): Promise<string> {
    return invoke<string>('read_config_file', { path });
  }

  return {
    readConfigFile,
  };
});
