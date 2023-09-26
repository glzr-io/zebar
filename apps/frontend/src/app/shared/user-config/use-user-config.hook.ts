import { createResource, createSignal } from 'solid-js';
import { parse } from 'yaml';
import { ZodError } from 'zod';

import { useDesktopCommands } from '../desktop';
import { useLogger } from '../logging';
import { UserConfig, UserConfigSchema } from './types/user-config.model';
import { memoize } from '../utils';
import { useConfigVariables } from './use-config-variables.hook';
import { getBarConfigs } from './utils/get-bar-configs';
import { createStore } from 'solid-js/store';
import { useProviderTree } from '../providers';

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

  const [configObj, { refetch: reload }] = createResource(async () => {
    try {
      // Read and parse the config as YAML.
      const config = await commands.readConfigFile();
      const configObj = parse(config) as unknown;

      logger.debug(`Read config:`, configObj);

      return configObj;
    } catch (err) {
      handleConfigError(err);
    }
  });

  const [config] = createResource(
    () => [configObj(), providerTree()] as const,
    async ([configObj, providerTree]) => {
      try {
        // Read and parse the config as YAML.
        const parsedConfig = await UserConfigSchema.parseAsync(configObj);
        logger.debug(`Parsed config:`, parsedConfig);

        return parsedConfig;
      } catch (err) {
        handleConfigError(err);
      }
    },
  );

  const [generalConfig] = createResource(config, config => config.general);

  const [barConfig] = createResource(config, config => {
    const barConfig = config[`bar/${barName()}`];

    if (!barConfig) {
      throw new Error(`Could not find bar config for '${barName()}'.`);
    }

    return barConfig;
  });

  // Handle errors in the user config file.
  function handleConfigError(err: unknown) {
    if (!(err instanceof Error)) {
      throw new Error('Problem reading config file.');
    }

    if (err instanceof ZodError) {
      const [firstError] = err.errors;
      const { path, message } = firstError;
      const fullPath = path.join('.');

      throw new Error(
        `Property '${fullPath}' in config isn't valid. Reason: '${message}'.`,
      );
    }

    throw new Error(`Problem reading config file: ${(err as Error).message}.`);
  }

  return {
    generalConfig,
    barConfig,
    reload,
  };
});
