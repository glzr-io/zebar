import { createResource } from 'solid-js';
import { parse } from 'yaml';

import { useLogger } from './logging/use-logger.hook';
import { useDesktopCommands } from './use-desktop-commands.hook';
import { memoize } from './utils/memoize';

export const useConfig = memoize(() => {
  const logger = useLogger('useConfig');
  const commands = useDesktopCommands();

  const [config] = createResource(async () => {
    const config = await commands.readConfigFile();

    // Parse the config as YAML.
    const parsedConfig = parse(config);
    logger.debug(`Read config:`, parsedConfig);

    return parsedConfig;
  });

  return config;
});
