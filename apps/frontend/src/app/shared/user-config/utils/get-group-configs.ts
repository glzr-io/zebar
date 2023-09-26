import { BarConfig } from '../types/bar/bar-config.model';
import { GroupConfig } from '../types/bar/group-config.model';

/**
 * Get group configs by filtering 'group/**' keys.
 **/
export function getGroupConfigs(barConfig: BarConfig) {
  return Object.entries(barConfig)
    .filter(([key, value]) => key.startsWith('group/') && !!value)
    .map(([_, value]) => value) as GroupConfig[];
}
