import { TemplateConfig } from '../types/window/template-config.model';
import { GroupConfig } from '../types/window/group-config.model';

/**
 * Object.entries() over 'template/**' keys.
 **/
export function getTemplateConfigEntries(groupConfig: GroupConfig) {
  return Object.entries(groupConfig).filter(
    ([key, value]) => key.startsWith('template/') && !!value,
  ) as [`template/${string}`, TemplateConfig][];
}

/**
 * Get template configs by filtering 'template/**' keys.
 **/
export function getTemplateConfigs(groupConfig: GroupConfig) {
  return getTemplateConfigEntries(groupConfig).map(([_, value]) => value);
}
