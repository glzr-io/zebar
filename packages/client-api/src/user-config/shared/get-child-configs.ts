import { WindowConfig, GroupConfig, TemplateConfig } from '..';

/**
 * Get child group or template element configs.
 */
export function getChildConfigs(
  config: WindowConfig | GroupConfig | TemplateConfig,
) {
  return Object.entries(config).filter(
    (
      entry,
    ): entry is
      | [`group/${string}`, GroupConfig]
      | [`template/${string}`, TemplateConfig] => {
      const [key] = entry;
      return key.startsWith('group/') || key.startsWith('template/');
    },
  );
}
