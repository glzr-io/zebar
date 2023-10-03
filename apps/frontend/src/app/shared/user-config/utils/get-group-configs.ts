import { BarConfig } from '../types/bar/bar-config.model';
import { GroupConfig } from '../types/bar/group-config.model';

/**
 * Object.entries() over 'group/**' keys.
 **/
export function getGroupConfigEntries(barConfig: BarConfig) {
  return Object.entries(barConfig).filter(
    ([key, value]) => key.startsWith('group/') && !!value,
  ) as [`group/${string}`, GroupConfig][];
}

/**
 * Get group configs by filtering 'group/**' keys.
 **/
export function getGroupConfigs(barConfig: BarConfig) {
  return getGroupConfigEntries(barConfig).map(([_, value]) => value);
}
