import { BarConfig } from '../types/bar/bar-config.model';
import { UserConfig } from '../types/user-config.model';

/**
 * Get group configs by filtering 'group/**' keys.
 **/
export function getBarConfigs(config: UserConfig) {
  return Object.entries(config)
    .filter(([key, value]) => key.startsWith('bar/') && !!value)
    .map(([_, value]) => value) as BarConfig[];
}
