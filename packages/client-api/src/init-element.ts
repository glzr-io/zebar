import { Accessor, createEffect } from 'solid-js';

import {
  WindowConfig,
  GroupConfig,
  TemplateConfig,
  useStyleBuilder,
} from './user-config';
import {
  ElementType,
  getParsedElementConfig,
  getElementVariables,
} from './context';
import { memoize } from './utils';

export interface InitElementArgs {
  id: string;
  config: WindowConfig | GroupConfig | TemplateConfig;
  ancestorVariables?: Accessor<Record<string, unknown>>[];
}

export const initElement = memoize((args: InitElementArgs) => {
  const styleBuilder = useStyleBuilder();
  const type = getElementType(args.id);

  const childConfigs = getChildConfigs(args.config);
  const childIds = childConfigs.map(([key]) => key);

  const { element, merged } = getElementVariables(
    args.config,
    args.ancestorVariables,
  );

  const parsedConfig = getParsedElementConfig({
    id: args.id,
    type,
    config: args.config,
    variables: merged,
  });

  createEffect(() => {
    if (parsedConfig.styles) {
      styleBuilder.setElementStyles(parsedConfig.id, parsedConfig.styles);
    }
  });

  return {
    id: args.id,
    rawConfig: args.config,
    parsedConfig,
    variables: merged,
    type,
    childIds,
    initChild: (id: string) => {
      const foundConfig = childConfigs.find(([key]) => key === id);

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
});

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
