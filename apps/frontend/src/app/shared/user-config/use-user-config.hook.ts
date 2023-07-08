import { createResource } from 'solid-js';
import { parse } from 'yaml';

import { useDesktopCommands } from '../desktop';
import { useLogger } from '../logging';
import { UserConfig } from './types/user-config.model';
import { memoize } from '../utils';

export const useUserConfig = memoize(() => {
  const logger = useLogger('useConfig');
  const commands = useDesktopCommands();

  const [config, { refetch: reload }] = createResource(async () => {
    const config = await commands.readConfigFile();

    // Parse the config as YAML.
    const parsedConfig = parse(config) as UserConfig;
    logger.debug(`Read config:`, parsedConfig);

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
