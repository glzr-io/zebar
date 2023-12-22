import { Accessor, Owner, createEffect, runWithOwner } from 'solid-js';

import {
  getStyleBuilder,
  getParsedElementConfig,
  getChildConfigs,
  GlobalConfig,
} from './user-config';
import { getElementProviders } from './providers';
import { ElementConfig, ElementContext } from './element-context.model';
import { ElementType } from './element-type.model';

export interface InitElementArgs {
  id: string;
  rawConfig: unknown;
  globalConfig: GlobalConfig;
  ancestorProviders: Accessor<Record<string, unknown>>[];
  owner: Owner;
}

export async function initElement(
  args: InitElementArgs,
): Promise<ElementContext> {
  const styleBuilder = getStyleBuilder(args.owner);
  const type = getElementType(args.id);
  const childConfigs = getChildConfigs(args.rawConfig as ElementConfig);

  // Create partial element context; `providers` and `parsedConfig` are set later.
  // TODO: Use something other than `Omit` to indicate that those fields are partial.
  // TODO: Add args and env to element context.
  const elementContext: Omit<
    ElementContext,
    'parsedConfig' | 'providers'
  > = {
    id: args.id,
    type,
    rawConfig: args.rawConfig,
    globalConfig: args.globalConfig,
    initChildElement,
  };

  const { element, merged } = await getElementProviders(
    elementContext,
    args.ancestorProviders,
    args.owner,
  );

  // Since `parsedConfig` is set after the element config is parsed, it is
  // initially unavailable on 'self' provider.
  elementContext.providers = merged;

  const parsedConfig = getParsedElementConfig(elementContext, args.owner);
  elementContext.parsedConfig = parsedConfig;

  runWithOwner(args.owner, () => {
    createEffect(() => {
      if (parsedConfig.styles) {
        styleBuilder.setElementStyles(
          parsedConfig.id,
          parsedConfig.styles,
        );
      }
    });
  });

  async function initChildElement(id: string) {
    const foundConfig = childConfigs.find(([key]) => key === id);

    if (!foundConfig) {
      return null;
    }

    const [configKey, childConfig] = foundConfig;

    return initElement({
      rawConfig: childConfig,
      id: configKey,
      ancestorProviders: [...(args.ancestorProviders ?? []), element],
      owner: args.owner,
      globalConfig: args.globalConfig,
    });
  }

  return elementContext;
}

function getElementType(id: string) {
  const [type] = id.split('/');

  // TODO: Validate in P1 schema instead.
  if (!Object.values(ElementType).includes(type as ElementType)) {
    throw new Error(`Unrecognized element type '${type}'.`);
  }

  return type as ElementType;
}
