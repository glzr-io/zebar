import {
  type Accessor,
  type Owner,
  createEffect,
  runWithOwner,
} from 'solid-js';
import { createStore } from 'solid-js/store';

import {
  getStyleBuilder,
  getParsedElementConfig,
  getChildConfigs,
  type GlobalConfig,
  getScriptManager,
} from './user-config';
import { getElementProviders } from './providers';
import type { ElementContext } from './element-context.model';
import { ElementType } from './element-type.model';
import { createLogger, type PickPartial } from './utils';
import { openWindow, showErrorDialog } from './desktop';

const logger = createLogger('init-element');

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
  try {
    const styleBuilder = getStyleBuilder();
    const scriptManager = getScriptManager();
    const childConfigs = getChildConfigs(args.rawConfig);

    // Create partial element context; `providers` and `parsedConfig` are set later.
    const [elementContext, setElementContext] = createStore<
      PickPartial<ElementContext, 'parsedConfig' | 'providers'>
    >({
      id: args.id,
      type: args.type,
      rawConfig: args.rawConfig,
      globalConfig: args.globalConfig,
      args: args.args,
      env: args.env,
      openWindow,
      initChildElement,
      providers: undefined,
      parsedConfig: undefined,
    });

    const { element, merged } = await getElementProviders(
      elementContext,
      args.ancestorProviders,
      args.owner,
    );

    setElementContext('providers', merged);

    const parsedConfig = getParsedElementConfig(
      elementContext as PickPartial<ElementContext, 'parsedConfig'>,
      args.owner,
    );

    // Since `parsedConfig` and `providers` are set after initializing providers
    // and parsing the element config, they are initially unavailable on 'self'
    // provider.
    setElementContext('parsedConfig', parsedConfig);

    // Build the CSS for the element.
    runWithOwner(args.owner, () => {
      createEffect(async () => {
        if (parsedConfig.styles) {
          try {
            styleBuilder.buildElementStyles(
              parsedConfig.id,
              parsedConfig.styles,
            );
          } catch (err) {
            await showErrorDialog({
              title: `Non-fatal: Error in ${args.type}/${args.id}`,
              error: err,
            });
          }
        }
      });
    });

    // Preload the scripts used for the element's events.
    runWithOwner(args.owner, () => {
      createEffect(async () => {
        try {
          await Promise.all(
            parsedConfig.events
              .map(config => config.fn_path)
              .map(scriptManager.loadScriptForFn),
          );
        } catch (err) {
          await showErrorDialog({
            title: `Non-fatal: Error in ${args.type}/${args.id}`,
            error: err,
          });
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
  } catch (err) {
    // Let error immediately bubble up if element is a window.
    if (args.type !== ElementType.WINDOW) {
      logger.error('Failed to initialize element:', err);

      await showErrorDialog({
        title: `Non-fatal: Error in ${args.type}/${args.id}`,
        error: err,
      });
    }

    throw err;
  }
}
