import { Accessor, Owner, createEffect, runWithOwner } from 'solid-js';

import {
  getStyleBuilder,
  getParsedElementConfig,
  GlobalConfig,
} from './user-config';
import { getElementProviders } from './providers';
import { ElementConfig, ElementContext } from './element-context.model';
import { ElementType } from './element-type.model';
import { getChildConfigs } from './user-config/shared/get-child-configs';

export interface InitElementArgs {
  id: string;
  rawConfig: ElementConfig;
  ancestorProviders: Accessor<Record<string, unknown>>[];
  owner: Owner;
  globalConfig: GlobalConfig;
}

export async function initElement(
  args: InitElementArgs,
): Promise<ElementContext> {
  const styleBuilder = getStyleBuilder(args.owner);
  const type = getElementType(args.id);

  const childConfigs = getChildConfigs(args.rawConfig);

  const elementContext = {
    id: args.id,
    type,
    rawConfig: args.rawConfig,
    globalConfig: args.globalConfig,
    initChildElement,
  } as ElementContext;

  const { element, merged } = await getElementProviders(
    args.rawConfig,
    args.ancestorProviders,
    args.owner,
  );

  const parsedConfig = getParsedElementConfig({
    id: args.id,
    type,
    rawConfig: args.rawConfig,
    providers: merged,
    owner: args.owner,
  });

  elementContext.providers = merged;
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
