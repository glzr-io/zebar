import { createStore } from 'solid-js/store';
import { parse } from 'yaml';

import { formatConfigError } from './utils/format-config-error';
import { createLogger } from '~/utils';
import { readConfigFile } from '~/desktop';

const logger = createLogger('create-config-store');

export interface ConfigStore {
  store: Record<string, unknown>;
  reload: () => Promise<void>;
}

/**
 * Get user config as parsed YAML.
 */
export async function createConfigStore(): Promise<ConfigStore> {
  const [configStore, setConfigStore] = createStore(await readConfig());

  // Read and parse the config as YAML.
  async function readConfig() {
    try {
      const configStr = await readConfigFile();
      const configObj = parse(configStr) as Record<string, unknown>;

      logger.debug(`Read config:`, configObj);

      return configObj;
    } catch (err) {
      throw formatConfigError(err);
    }
  }

  async function reload() {
    setConfigStore(await readConfig());
  }

  return {
    store: configStore,
    reload,
  };
}
