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

    const expandConfig = expandObjectKeys(parsedConfig, [
      'bar',
      'group',
      'slot',
    ]);

    logger.debug(`Expanded config:`, expandConfig);

    // const transformedConfig =
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

/**
 * Expand 'bar/', 'group/', and 'slot/' keys within config.

 * @example
 * ```typescript
 * expandKeysToObject({ 'bar/main': { ... } }) // -> { bar: { main: { ... } } }
 * ```
 * */
export function expandObjectKeys(
  value: unknown | unknown[],
  keysToExpand: string[],
): unknown | unknown[] {
  // Ignore values that cannot be further traversed.
  if (!(isPlainObject(value) || Array.isArray(value))) {
    return value;
  }

  if (Array.isArray(value)) {
    return value.map(item => expandObjectKeys(item, keysToExpand));
  }

  return Object.keys(value).reduce((acc, key) => {
    const shouldExpand = keysToExpand.some(keyToExpand =>
      key.startsWith(keyToExpand),
    );

    // If key shouldn't be expanded, continue traversing.
    if (!shouldExpand) {
      return {
        ...acc,
        [key]: expandObjectKeys((value as any)[key], keysToExpand),
      };
    }

    const [mainKey, subKey] = key.split('/');

    return {
      ...acc,
      [mainKey]: {
        ...(acc?.[mainKey] ?? {}),
        [subKey]: expandObjectKeys((value as any)[key], keysToExpand),
      },
    };
  }, {} as Record<string, unknown>);
}

/** Whether given value is an object literal. */
export function isPlainObject(value: unknown): value is object {
  return value instanceof Object && !(value instanceof Array);
}
