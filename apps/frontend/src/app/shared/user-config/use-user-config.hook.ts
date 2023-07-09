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

    const expandConfig = expandKeysToObject(parsedConfig, [
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

/** An object that can be traversed, allowing nested objects and arrays. */
type Traversable = {
  [key: string]: unknown | Traversable | Traversable[];
};

/**
 * Expand 'bar/', 'group/', and 'slot/' keys within config.

 * @example
 * ```typescript
 * expandKeysToObject({ 'bar/main': { ... } }) // -> { bar: { main: { ... } } }
 * ```
 * */
export function expandKeysToObject<T extends Traversable>(
  obj: T,
  keysToExpand: string[],
): T {
  return Object.keys(obj).reduce((acc, key) => {
    const shouldExpand = keysToExpand.some(e => key.startsWith(e));

    if (shouldExpand) {
      const [mainKey, subKey] = key.split('/');
      const expandedValue = expandKeysToObject(
        obj[key] as Traversable,
        keysToExpand,
      );

      return {
        ...acc,
        [mainKey]: {
          ...(acc?.[mainKey] ?? {}),
          [subKey]: expandedValue,
        },
      };
    } else {
      return {
        ...acc,
        [key]:
          typeof obj[key] === 'object'
            ? expandKeysToObject(obj[key] as Traversable, keysToExpand)
            : obj[key],
      };
    }
  }, {} as T);
}

// v1
function expandConfig(config) {
  return {
    ...config,
    // Expand 'bar/**' keys to object.
    bars: keysWithPrefix(config, 'bar/').reduce((acc, key) => {
      const barConfig = config[key];
      const barConfigKey = key.replace('bar/', '');

      return {
        ...acc,
        [barConfigKey]: {
          ...barConfig,
          // Expand 'group/**' keys to object.
          groups: keysWithPrefix(barConfig, `group/${key}/`).reduce(
            (acc, key) => {
              const groupConfig = barConfig[key];
              const groupConfigKey = key.replace('group/', '');

              return {
                ...acc,
                [groupConfigKey]: {
                  ...groupConfig,
                  // Expand 'slot/**' keys to object.
                  components: keysWithPrefix(config, `slot/${key}/`).reduce(
                    (acc, key) => {},
                    {},
                  ),
                  // components: config[key].components.map(component => {
                  //   return {
                  //     ...component,
                  //     slot: {
                  //       ...(component.slot ? { default: component.slot } : {}),
                  //       // ...
                  //     },
                  //   };
                  // }),
                },
              };
            },
            {},
          ),
        },
      };
    }, {}),
  };
}

export function keysWithPrefix(obj, prefix) {
  return Object.keys(obj).filter(key => key.startsWith(prefix));
}
