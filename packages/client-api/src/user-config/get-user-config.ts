import { createResource } from 'solid-js';
import { parse } from 'yaml';

import { formatConfigError } from './utils/format-config-error';
import { createLogger, memoize } from '~/utils';
import { readConfigFile } from '~/desktop';

const logger = createLogger('get-user-config');

/**
 * Get user config as parsed YAML.
 */
export const getUserConfig = memoize(() => {
  const [config, { refetch: reload }] = createResource(readUserConfig);

  // Read and parse the config as YAML.
  async function readUserConfig() {
    try {
      const configStr = await readConfigFile();
      const configObj = parse(configStr) as unknown;

      logger.debug(`Read config:`, configObj);

      return configObj;
    } catch (err) {
      throw formatConfigError(err);
    }
  }

  return {
    config,
    reload,
  };
});
