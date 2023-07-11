import { createResource, createSignal } from 'solid-js';
import { parse } from 'yaml';

import { useDesktopCommands } from '../desktop';
import { useLogger } from '../logging';
import { UserConfigSchema } from './types/user-config.model';
import { Prettify, memoize } from '../utils';

export const useUserConfig = memoize(() => {
  const logger = useLogger('useConfig');
  const commands = useDesktopCommands();

  // TODO: Get name of bar from launch args. Default to 'default.'
  const [barName] = createSignal('default');

  const [config, { refetch: reload }] = createResource(async () => {
    try {
      const config = await commands.readConfigFile();

      // Parse the config as YAML.
      const configObj = parse(config) as unknown;
      logger.debug(`Read config:`, configObj);

      const parsedConfig = await UserConfigSchema.parseAsync(configObj);
      logger.debug(`Parsed config:`, parsedConfig);

      return parsedConfig;
    } catch (err) {
      commands.exitWithError(`Problem reading config file: ${err}`);
    }
  });

  const [generalConfig] = createResource(config, config => config.general);

  const [barConfig] = createResource(config, config => {
    const barConfig = config[`bar/${barName()}`];

    if (!barConfig) {
      commands.exitWithError(`Could not find bar config for '${barName()}'.`);
    }

    return barConfig;
  });

  return {
    generalConfig,
    barConfig,
    reload,
  };
});
