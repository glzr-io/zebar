import { WindowConfig } from '../types/window/window-config.model';
import { UserConfig } from '../types/user-config.model';

/**
 * Object.entries() over 'window/**' keys.
 **/
export function getBarConfigEntries(config: UserConfig) {
  return Object.entries(config).filter(
    ([key, value]) => key.startsWith('window/') && !!value,
  ) as [`window/${string}`, WindowConfig][];
}

/**
 * Get bar configs by filtering 'window/**' keys.
 **/
export function getBarConfigs(config: UserConfig) {
  return getBarConfigEntries(config).map(([_, value]) => value);
}
