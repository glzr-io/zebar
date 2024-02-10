import type { ElementConfig } from '~/element-context.model';
import type { GroupConfig, TemplateConfig } from '../window';
import type { ElementType } from '~/element-type.model';

export interface ChildConfigRef {
  type: ElementType;
  id: string;
  config: TemplateConfig | GroupConfig;
}

/**
 * Get template and group configs within {@link rawConfig}.
 */
export function getChildConfigs(rawConfig: unknown): ChildConfigRef[] {
  return Object.entries(rawConfig as ElementConfig).reduce<
    ChildConfigRef[]
  >((acc, [key, value]) => {
    const childKeyRegex = /^(template|group)\/(.+)$/;
    const match = key.match(childKeyRegex);

    if (!match) {
      return acc;
    }

    return [
      ...acc,
      {
        type: match[1],
        id: match[2],
        config: value,
      } as ChildConfigRef,
    ];
  }, []);
}
