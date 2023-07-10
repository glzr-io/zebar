import { plainToInstance } from 'class-transformer';
import { validate } from 'class-validator';
import { createResource } from 'solid-js';
import { parse } from 'yaml';

import { useDesktopCommands } from '../desktop';
import { useLogger } from '../logging';
import { UserConfig } from './types/user-config.model';
import { memoize } from '../utils';
import { expandConfigKeys } from './expand-config-keys';

export const useUserConfig = memoize(() => {
  const logger = useLogger('useConfig');
  const commands = useDesktopCommands();

  const [config, { refetch: reload }] = createResource(async () => {
    const config = await commands.readConfigFile();

    // Parse the config as YAML.
    const parsedConfig = parse(config);
    logger.debug(`Read config:`, parsedConfig);

    const expandedConfig = expandConfigKeys(parsedConfig, [
      'bar',
      'group',
      'slot',
    ]);

    const configInstance = plainToInstance(UserConfig, expandedConfig);
    logger.debug(`Expanded config:`, configInstance);

    const validationErrors = await validate(configInstance);

    if (validationErrors.length > 0) {
      logger.error(`Config errors:`, validationErrors);
    }

    // TODO: Traverse config and add IDs to each component.
    // TODO: Traverse config and aggregate `styles`. Compile this and
    // add it to the DOM somehow.

    return parsedConfig;
  });

  const [generalConfig] = createResource(config, config => config.general);
  const [barConfig] = createResource(config, config => config['bar/main']);

  return {
    generalConfig,
    barConfig,
    reload,
  };
});
