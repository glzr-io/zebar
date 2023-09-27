import { createResource } from 'solid-js';
import { parse } from 'yaml';

import { useDesktopCommands } from '../desktop';
import { useLogger } from '../logging';
import { memoize } from '../utils';
import { formatConfigError } from './utils/format-config-error';
import { createConfigStore } from './utils/create-config-store';

// In bar.component.ts:
// const providerTree = useProviderTree();
// const providers = providerTree.find(node => node.id === config.id);

// But does bar.component.ts actually care about the provider tree? It just
// cares about the transformed config values.

// How to deal with `env` variables in

export const useUserConfig = memoize(() => {
  const logger = useLogger('useConfig');
  const commands = useDesktopCommands();

  const [configObj, { refetch: reload }] = createResource(() =>
    readUserConfig(),
  );

  const config = createConfigStore(configObj);

  const [currentBarConfig] = createResource(
    () => config.value,
    config => {
      // TODO: Get name of bar from launch args. Default to 'default.'
      const barName = 'default';
      const barConfig = config[`bar/${barName}`];

      if (!barConfig) {
        throw new Error(`Could not find bar config for '${barName}'.`);
      }

      return barConfig;
    },
  );

  // Read and parse the config as YAML.
  async function readUserConfig() {
    try {
      const config = await commands.readConfigFile();
      const configObj = parse(config) as unknown;

      logger.debug(`Read config:`, configObj);

      return configObj;
    } catch (err) {
      throw formatConfigError(err);
    }
  }

  return {
    get config() {
      return config.value;
    },
    currentBarConfig,
    reload,
  };
});
