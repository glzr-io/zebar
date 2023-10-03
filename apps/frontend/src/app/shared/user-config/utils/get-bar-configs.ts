import { BarConfig } from '../types/bar/bar-config.model';
import { UserConfig } from '../types/user-config.model';

/**
 * Object.entries() over 'bar/**' keys.
 **/
export function getBarConfigEntries(config: UserConfig) {
  return Object.entries(config).filter(
    ([key, value]) => key.startsWith('bar/') && !!value,
  ) as [`bar/${string}`, BarConfig][];
}

/**
 * Get bar configs by filtering 'bar/**' keys.
 **/
export function getBarConfigs(config: UserConfig) {
  return getBarConfigEntries(config).map(([_, value]) => value);
}
