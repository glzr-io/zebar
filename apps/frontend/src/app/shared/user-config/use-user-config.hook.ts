import { createResource, createSignal } from 'solid-js';
import { parse } from 'yaml';

import { useDesktopCommands } from '../desktop';
import { useLogger } from '../logging';
import { UserConfigSchema } from './types/user-config.model';
import { memoize } from '../utils';
import { useProviderTree } from '../providers';
import { formatConfigError } from './utils/format-config-error';

// In bar.component.ts:
// const providerTree = useProviderTree();
// const providers = providerTree.find(node => node.id === config.id);

// But does bar.component.ts actually care about the provider tree? It just
// cares about the transformed config values.

// How to deal with `env` variables in

export const useUserConfig = memoize(() => {
  const logger = useLogger('useConfig');
  const commands = useDesktopCommands();
  const providerTree = useProviderTree();

  // TODO: Get name of bar from launch args. Default to 'default.'
  const [barName] = createSignal('default');

  const [config, { refetch: reload }] = createResource(async () => {
    try {
      const config = await commands.readConfigFile();

      // Parse the config as YAML.
      const configObj = parse(config) as unknown;
      logger.debug(`Read config:`, configObj);

      const tree = providerTree.update(configObj);
      console.log('providerTree', providerTree);

      // Need to somehow traverse down config and compile all templates.
      const parsedConfig = await UserConfigSchema.parseAsync(configObj);
      logger.debug(`Parsed config:`, parsedConfig);

      return parsedConfig;
    } catch (err) {
      throw formatConfigError(err);
    }
  });

  const [generalConfig] = createResource(config, config => config.general);

  const [barConfig] = createResource(config, config => {
    const barConfig = config[`bar/${barName()}`];

    if (!barConfig) {
      throw new Error(`Could not find bar config for '${barName()}'.`);
    }

    return barConfig;
  });

  return {
    generalConfig,
    barConfig,
    reload,
  };
});
