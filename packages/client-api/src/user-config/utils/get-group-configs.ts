import { WindowConfig } from '../types/window/window-config.model';
import { GroupConfig } from '../types/window/group-config.model';

/**
 * Object.entries() over 'group/**' keys.
 **/
export function getGroupConfigEntries(barConfig: WindowConfig) {
  return Object.entries(barConfig).filter(
    ([key, value]) => key.startsWith('group/') && !!value,
  ) as [`group/${string}`, GroupConfig][];
}

/**
 * Get group configs by filtering 'group/**' keys.
 **/
export function getGroupConfigs(barConfig: WindowConfig) {
  return getGroupConfigEntries(barConfig).map(([_, value]) => value);
}
