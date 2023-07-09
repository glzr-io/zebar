import { isObject } from '../utils';
import { BarConfig } from './types/bar/bar-config.model';
import { ComponentConfig } from './types/bar/component-config.model';
import { ComponentGroupConfig } from './types/bar/component-group-config.model';
import { GeneralConfig } from './types/general-config.model';
import { UserConfig } from './types/user-config.model';

type NestedConfig =
  | UserConfig
  | GeneralConfig
  | BarConfig
  | ComponentGroupConfig
  | ComponentConfig;

/** Force a type to be indexable. */
type Indexable = Record<string, any>;

/**
 * Expand `bar/`, `group/`, and `slot/` keys within config.
 *
 * @example
 * ```typescript
 * expandConfigKeys({ 'bar/main': { ... } }) // -> { bar: { main: { ... } } }
 * ```
 * */
export function expandConfigKeys<T extends NestedConfig>(
  config: T,
  keysToExpand: string[],
): T {
  return Object.keys(config).reduce((acc, key) => {
    const shouldExpand = keysToExpand.some(keyToExpand =>
      key.startsWith(keyToExpand),
    );

    // If key shouldn't be expanded, assign key as usual.
    if (!shouldExpand) {
      return {
        ...acc,
        [key]: getNestedConfigValue(config, key, keysToExpand),
      };
    }

    const [mainKey, subKey] = key.split('/');

    // Otherwise, assign key as nested object.
    return {
      ...acc,
      [mainKey]: {
        ...((acc as Indexable)?.[mainKey] ?? {}),
        [subKey]: getNestedConfigValue(config, key, keysToExpand),
      },
    };
  }, {} as T);
}

function getNestedConfigValue<T extends NestedConfig>(
  config: T,
  key: string,
  keysToExpand: string[],
): unknown {
  const value = (config as Indexable)[key];

  if (isObject(value)) {
    return expandConfigKeys(value as NestedConfig, keysToExpand);
  }

  if (Array.isArray(value)) {
    return value.map(value => expandConfigKeys(value, keysToExpand));
  }

  return value;
}
