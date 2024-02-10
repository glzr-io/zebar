import {
  type Accessor,
  type Owner,
  createEffect,
  runWithOwner,
} from 'solid-js';

import {
  getStyleBuilder,
  getParsedElementConfig,
  getChildConfigs,
  type GlobalConfig,
} from './user-config';
import { getElementProviders } from './providers';
import type { ElementContext } from './element-context.model';
import { ElementType } from './element-type.model';
import type { PickPartial } from './utils';

export interface InitElementArgs {
  id: string;
  type: ElementType;
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
  const childConfigs = getChildConfigs(args.rawConfig);

  // Create partial element context; `providers` and `parsedConfig` are set later.
  const elementContext: PickPartial<
    ElementContext,
    'parsedConfig' | 'providers'
  > = {
    id: args.id,
    type: args.type,
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
    const childConfig = childConfigs.find(
      childConfig => childConfig.id === id,
    );

    // Check whether an element with the given ID exists in the config.
    if (!childConfig) {
      return null;
    }

    return initElement({
      id,
      type: childConfig.type,
      rawConfig: childConfig.config,
      globalConfig: args.globalConfig,
      args: args.args,
      env: args.env,
      ancestorProviders: [...(args.ancestorProviders ?? []), element],
      owner: args.owner,
    });
  }

  return elementContext as ElementContext;
}
