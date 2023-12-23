import { ElementConfig } from '~/element-context.model';
import { GroupConfig, TemplateConfig } from '../window';

/**
 * Get ID's of child group or template elements from an unparsed config.
 */
export function getChildIds(rawConfig: unknown) {
  const childEntries = Object.entries(rawConfig as ElementConfig).filter(
    (
      entry,
    ): entry is
      | [`group/${string}`, GroupConfig]
      | [`template/${string}`, TemplateConfig] => {
      const [key] = entry;
      return key.startsWith('group/') || key.startsWith('template/');
    },
  );

  return childEntries.map(([id]) => id);
}
