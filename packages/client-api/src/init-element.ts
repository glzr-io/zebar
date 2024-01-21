import {
  type Accessor,
  type Owner,
  createEffect,
  runWithOwner,
} from 'solid-js';

import {
  getStyleBuilder,
  getParsedElementConfig,
  getChildIds,
  type GlobalConfig,
} from './user-config';
import { getElementProviders } from './providers';
import type { ElementContext } from './element-context.model';
import { ElementType } from './element-type.model';
import type { PickPartial } from './utils';

export interface InitElementArgs {
  id: string;
  rawConfig: unknown;
  globalConfig: GlobalConfig;
  args: Record<string, string>;
  env: Record<string, string>;
  ancestorProviders: Accessor<Record<string, unknown>>[];
  owner: Owner;
}

export async function initElement(
  args: InitElementArgs,
): Promise<ElementContext> {
  const styleBuilder = getStyleBuilder(args.owner);
  const type = getElementType(args.id);
  const childIds = getChildIds(args.rawConfig);

  // Create partial element context; `providers` and `parsedConfig` are set later.
  const elementContext: PickPartial<
    ElementContext,
    'parsedConfig' | 'providers'
  > = {
    id: args.id,
    type,
    rawConfig: args.rawConfig,
    globalConfig: args.globalConfig,
    args: args.args,
    env: args.env,
    initChildElement,
  };

  const { element, merged } = await getElementProviders(
    elementContext,
    args.ancestorProviders,
    args.owner,
  );

  elementContext.providers = merged;

  const parsedConfig = getParsedElementConfig(
    elementContext as PickPartial<ElementContext, 'parsedConfig'>,
    args.owner,
  );

  // Since `parsedConfig` and `providers` are set after initializing providers
  // and parsing the element config, they are initially unavailable on 'self'
  // provider.
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
    // Check whether an element with the given ID exists in the config.
    if (!childIds.find(childId => childId === id)) {
      return null;
    }

    return initElement({
      id,
      rawConfig: (args.rawConfig as Record<string, unknown>)[id],
      globalConfig: args.globalConfig,
      args: args.args,
      env: args.env,
      ancestorProviders: [...(args.ancestorProviders ?? []), element],
      owner: args.owner,
    });
  }

  return elementContext as ElementContext;
}

function getElementType(id: string) {
  const [type] = id.split('/');

  // TODO: Validate in P1 schema instead.
  if (!Object.values(ElementType).includes(type as ElementType)) {
    throw new Error(`Unrecognized element type '${type}'.`);
  }

  return type as ElementType;
}
