import { ComponentConfig } from '../types/bar/component-config.model';
import { GroupConfig } from '../types/bar/group-config.model';

/**
 * Object.entries() over 'component/**' keys.
 **/
export function getComponentConfigEntries(groupConfig: GroupConfig) {
  return Object.entries(groupConfig).filter(
    ([key, value]) => key.startsWith('component/') && !!value,
  ) as [`component/${string}`, ComponentConfig][];
}

/**
 * Get component configs by filtering 'component/**' keys.
 **/
export function getComponentConfigs(groupConfig: GroupConfig) {
  return getComponentConfigEntries(groupConfig).map(([_, value]) => value);
}
