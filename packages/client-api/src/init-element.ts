import { Accessor } from 'solid-js';

import { WindowConfig, GroupConfig, TemplateConfig } from './user-config';
import { ElementType, getElementVariables } from './context';

export interface InitElementArgs {
  id: string;
  config: WindowConfig | GroupConfig | TemplateConfig;
  ancestorVariables?: Accessor<Record<string, unknown>>[];
}

export function initElement(args: InitElementArgs) {
  const type = getElementType(args.id);

  const childConfigs = getChildConfigs(args.config);
  const childIds = childConfigs.map(([key]) => key);

  const { element, merged } = getElementVariables(args.config);

  return {
    id: args.id,
    rawConfig: args.config,
    parsedConfig,
    variables: merged,
    type,
    childIds,
    initChild: () => {
      const foundConfig = childConfigs.find(([key]) => key === args.id);

      if (!foundConfig) {
        return null;
      }

      const [configKey, childConfig] = foundConfig;

      return initElement({
        config: childConfig,
        id: configKey,
        ancestorVariables: [...(args.ancestorVariables ?? []), element],
      });
    },
  };
}

/**
 * Get child element configs.
 */
function getChildConfigs(config: WindowConfig | GroupConfig | TemplateConfig) {
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

function getElementType(id: string) {
  const [type] = id.split('/');

  // TODO: Validate in P1 schema instead.
  if (!Object.values(ElementType).includes(type as ElementType)) {
    throw new Error(`Unrecognized element type '${type}'.`);
  }

  return type as ElementType;
}
