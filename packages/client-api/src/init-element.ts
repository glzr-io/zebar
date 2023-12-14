import { Accessor, Owner, createEffect, runWithOwner } from 'solid-js';

import {
  WindowConfig,
  GroupConfig,
  TemplateConfig,
  getStyleBuilder,
  getParsedElementConfig,
} from './user-config';
import { ElementContext, ElementType, getElementVariables } from './context';

export interface InitElementArgs {
  id: string;
  config: WindowConfig | GroupConfig | TemplateConfig;
  ancestorVariables: Accessor<Record<string, unknown>>[];
  owner: Owner;
}

export async function initElement(
  args: InitElementArgs,
): Promise<ElementContext> {
  const styleBuilder = getStyleBuilder(args.owner);
  const type = getElementType(args.id);

  const childConfigs = getChildConfigs(args.config);
  const childIds = childConfigs.map(([key]) => key);

  const { element, merged } = await getElementVariables(
    args.config,
    args.ancestorVariables,
    args.owner,
  );

  const parsedConfig = getParsedElementConfig({
    id: args.id,
    type,
    config: args.config,
    variables: merged,
    owner: args.owner,
  });

  runWithOwner(args.owner, () => {
    createEffect(() => {
      if (parsedConfig.styles) {
        styleBuilder.setElementStyles(parsedConfig.id, parsedConfig.styles);
      }
    });
  });

  async function initChild(id: string) {
    const foundConfig = childConfigs.find(([key]) => key === id);

    if (!foundConfig) {
      return null;
    }

    const [configKey, childConfig] = foundConfig;

    return initElement({
      config: childConfig,
      id: configKey,
      ancestorVariables: [...(args.ancestorVariables ?? []), element],
      owner: args.owner,
    });
  }

  return {
    id: args.id,
    rawConfig: args.config,
    parsedConfig,
    variables: merged,
    type,
    childIds,
    initChild,
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
