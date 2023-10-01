import { BarConfig } from '../types/bar/bar-config.model';
import { UserConfig } from '../types/user-config.model';

/**
 * Get bar configs by filtering 'bar/**' keys.
 **/
export function getBarConfigs(config: UserConfig) {
  return Object.entries(config).filter(
    ([key, value]) => key.startsWith('bar/') && !!value,
  ) as [`bar/${string}`, BarConfig][];
}
