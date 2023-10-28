import { createResource } from 'solid-js';
import { parse } from 'yaml';

import { formatConfigError } from './shared';
import { createLogger } from '~/utils';
import { readConfigFile } from '~/desktop';

const logger = createLogger('get-user-config');

/**
 * Get user config as parsed YAML.
 */
export function getUserConfig() {
  return createResource(async () => {
    try {
      const configStr = await readConfigFile();
      const configObj = parse(configStr) as unknown;

      logger.debug(`Read config:`, configObj);

      return configObj;
    } catch (err) {
      throw formatConfigError(err);
    }
  });
}
