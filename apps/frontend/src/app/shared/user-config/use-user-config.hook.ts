import { createResource, createSignal } from 'solid-js';
import { parse } from 'yaml';
import { ZodError } from 'zod';

import { useDesktopCommands } from '../desktop';
import { useLogger } from '../logging';
import { getBindingRegex } from '../template-parsing';
import { UserConfigSchema } from './types/user-config.model';
import { memoize } from '../utils';
import { useConfigVariables } from './use-config-variables.hook';

export const useUserConfig = memoize(() => {
  const logger = useLogger('useConfig');
  const commands = useDesktopCommands();
  const configVariables = useConfigVariables();

  // TODO: Get name of bar from launch args. Default to 'default.'
  const [barName] = createSignal('default');

  const [config, { refetch: reload }] = createResource(
    configVariables,
    evaluateUserConfig,
  );

  const [generalConfig] = createResource(config, config => config.general);

  const [barConfig] = createResource(config, config => {
    const barConfig = config[`bar/${barName()}`];

    if (!barConfig) {
      commands.exitWithError(`Could not find bar config for '${barName()}'.`);
    }

    return barConfig;
  });

  // Read and parse user config file.
  async function evaluateUserConfig(configVariables: Record<string, string>) {
    try {
      let config = await commands.readConfigFile();

      // Prior to parsing, replace any config variables found.
      for (const [name, value] of Object.entries(configVariables)) {
        config = config.replace(
          getBindingRegex(`vars.${name}`),
          value.toString(),
        );
      }

      // Parse the config as YAML.
      const configObj = parse(config) as unknown;
      logger.debug(`Read config:`, configObj);

      const parsedConfig = await UserConfigSchema.parseAsync(configObj);
      logger.debug(`Parsed config:`, parsedConfig);

      return parsedConfig;
    } catch (err) {
      handleConfigError(err);
    }
  }

  // Handle errors in the user config file.
  function handleConfigError(err: unknown) {
    if (!(err instanceof Error)) {
      commands.exitWithError('Problem reading config file.');
    }

    if (err instanceof ZodError) {
      const [firstError] = err.errors;
      const { path, message } = firstError;
      const fullPath = path.join('.');

      commands.exitWithError(
        `Property '${fullPath}' in config isn't valid. Reason: '${message}'.`,
      );
    }

    commands.exitWithError(
      `Problem reading config file: ${(err as Error).message}.`,
    );
  }

  return {
    generalConfig,
    barConfig,
    reload,
  };
});
