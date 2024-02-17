import { createSignal } from 'solid-js';
import { YAMLParseError, parse } from 'yaml';

import { formatConfigError } from './shared';
import { createLogger } from '~/utils';
import { readConfigFile } from '~/desktop';

const logger = createLogger('get-user-config');

/**
 * User config (if read) as parsed YAML.
 */
const [userConfig, setUserConfig] = createSignal<unknown | null>(null);

/**
 * Get user config as parsed YAML.
 */
export async function getUserConfig() {
  if (userConfig()) {
    return userConfig();
  }

  try {
    const configStr = await readConfigFile();
    const configObj = parse(configStr) as unknown;
    setUserConfig(configObj);

    logger.debug(`Read config:`, configObj);

    return configObj;
  } catch (err) {
    throw new Error(
      `Problem reading config file: ${(err as Error).message}`,
    );
  }
}
